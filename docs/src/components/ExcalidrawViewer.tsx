import { useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import '@excalidraw/excalidraw/index.css';
import '../styles/fullscreen-modal.css'; // Shared modal styles

/**
 * Embeds an Excalidraw diagram from a .excalidraw JSON URL.
 * Loads the scene and renders it in view-only mode.
 * Elements with a "link" property in the JSON are clickable and navigate to docs.
 */
export interface ExcalidrawViewerProps {
  /** URL to the .excalidraw JSON file (e.g. /images/Architecture/system-architecture.excalidraw) */
  src: string;
  /** Height of the canvas in pixels or CSS value (default: 480) */
  height?: number | string;
}

interface ExcalidrawScene {
  type?: string;
  version?: number;
  source?: string;
  elements: unknown[];
  appState?: Record<string, unknown>;
  files?: Record<string, unknown>;
}

export function ExcalidrawViewer({ src, height = 480 }: ExcalidrawViewerProps) {
  const [scene, setScene] = useState<ExcalidrawScene | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [ExcalidrawComponent, setExcalidrawComponent] = useState<React.ComponentType<any> | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [mounted, setMounted] = useState(false);

  const [excalidrawAPI, setExcalidrawAPI] = useState<any>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Resolve src against current origin so fetch works with any base path
  const resolvedSrc = typeof window !== 'undefined' && src.startsWith('/')
    ? `${window.location.origin}${src}`
    : src;

  // Hide this instance's Astro placeholder (use closest wrapper so multiple viewers work)
  useEffect(() => {
    containerRef.current?.closest('.excalidraw-viewer-wrapper')?.setAttribute('data-loaded', 'true');
  }, []);

  // Dynamic import so Excalidraw (large) only loads when this component mounts
  useEffect(() => {
    let cancelled = false;
    const timeout = window.setTimeout(() => {
      if (cancelled) return;
      setError('Diagram took too long to load. Check the browser console and that the diagram file is reachable.');
    }, 20000);

    Promise.all([
      import('@excalidraw/excalidraw').then((m) => m.Excalidraw),
      fetch(resolvedSrc).then((r) => (r.ok ? r.json() : Promise.reject(new Error(`Failed to load ${src} (${r.status})`)))),
    ])
      .then(([Excalidraw, data]) => {
        if (cancelled) return;
        window.clearTimeout(timeout);
        setExcalidrawComponent(() => Excalidraw);
        const sceneData = data as ExcalidrawScene;
        if (!sceneData.elements || !Array.isArray(sceneData.elements)) {
          setError('Invalid .excalidraw file: missing elements array');
          return;
        }
        const appState = (sceneData.appState ?? {}) as Record<string, unknown>;
        setScene({
          elements: sceneData.elements,
          appState: {
            ...appState,
            scrollToContent: true,
          },
          files: sceneData.files ?? {},
        });
      })
      .catch((err) => {
        if (cancelled) return;
        window.clearTimeout(timeout);
        setError(err?.message ?? 'Failed to load diagram');
      });

    return () => {
      cancelled = true;
      window.clearTimeout(timeout);
    };
  }, [resolvedSrc, src]);

  // Zoom to fit content once loaded
  useEffect(() => {
    if (excalidrawAPI && scene) {
      excalidrawAPI.scrollToContent(scene.elements, { fitToViewport: true });
    }
  }, [excalidrawAPI, scene]);

  // Escape closes fullscreen
  useEffect(() => {
    if (isFullscreen) {
      document.body.classList.add('excalidraw-fullscreen-active');
      const onKeyDown = (e: KeyboardEvent) => {
        if (e.key === 'Escape') setIsFullscreen(false);
      };
      window.addEventListener('keydown', onKeyDown);
      return () => {
        document.body.classList.remove('excalidraw-fullscreen-active');
        window.removeEventListener('keydown', onKeyDown);
      };
    } else {
      document.body.classList.remove('excalidraw-fullscreen-active');
    }
  }, [isFullscreen]);

  // Ensure component is mounted (client-side) before rendering Portal
  useEffect(() => {
    setMounted(true);
  }, []);

  if (error) {
    return (
      <div ref={containerRef} className="excalidraw-viewer-error" style={{
        padding: '1rem',
        border: '1px solid rgba(248, 113, 113, 0.6)',
        borderRadius: '8px',
        color: '#fca5a5',
        fontSize: '0.9rem',
        minHeight: typeof height === 'number' ? `${height}px` : height,
        background: 'rgba(30, 30, 30, 0.9)',
      }}>
        Diagram could not be loaded: {error}. Check that <code>{src}</code> exists and is valid JSON.
      </div>
    );
  }

  if (!ExcalidrawComponent || !scene) {
    return (
      <div ref={containerRef} className="excalidraw-viewer-loading" style={{
        height: typeof height === 'number' ? `${height}px` : height,
        minHeight: 320,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        background: 'rgba(255, 255, 255, 0.05)',
        border: '1px dashed rgba(255, 255, 255, 0.15)',
        borderRadius: '8px',
        fontSize: '0.95rem',
        color: 'rgba(255, 255, 255, 0.75)',
      }}>
        Loading diagram…
      </div>
    );
  }

  // Single Excalidraw instance — light theme, original diagram colors
  const excalidrawEl = (
    <ExcalidrawComponent
      initialData={{
        elements: scene.elements,
        appState: { ...scene.appState, scrollToContent: true },
        scrollToContent: true,
      }}
      viewModeEnabled={true}
      theme="light"
      excalidrawAPI={(api: any) => setExcalidrawAPI(api)}
    />
  );

  const heightPx = typeof height === 'number' ? `${height}px` : height;
  const containerStyle: React.CSSProperties = {
    position: 'relative',
    height: heightPx,
    width: '100%',
    minHeight: 320,
    overflow: 'hidden',
    isolation: 'isolate',
  };

  // Render modal via Portal directly into document.body
  const modalContent = isFullscreen && mounted && typeof document !== 'undefined' && document.body ? (
    <>
      {/* Backdrop - uses shared .fs-backdrop class */}
      <div
        className="fs-backdrop"
        onClick={() => setIsFullscreen(false)}
      />

      {/* Modal Dialog - uses shared .fs-modal class with .is-open state */}
      <div
        className="fs-modal is-open"
        role="dialog"
        aria-modal="true"
        aria-label="Diagram expanded view"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header - uses shared .fs-modal-header */}
        <div className="fs-modal-header">
          <span className="fs-modal-title">
            Diagram (Expanded View)
          </span>
          <button
            type="button"
            className="fs-btn-close"
            onClick={() => setIsFullscreen(false)}
          >
            ✕ Close (ESC)
          </button>
        </div>

        {/* content - uses shared .fs-modal-body */}
        <div className="fs-modal-body" style={{ padding: 0 }}>
          <div style={{ width: '100%', height: '100%' }}>
            {excalidrawEl}
          </div>
        </div>
      </div>
    </>
  ) : null;

  return (
    <>
      <div ref={containerRef} className="excalidraw-viewer" style={containerStyle}>
        <div className="excalidraw-canvas-wrapper">
          <button
            type="button"
            onClick={() => setIsFullscreen(true)}
            className="fs-btn-open"
            title="Fullscreen"
            aria-label="Open diagram in fullscreen"
          >
            ⛶ Fullscreen
          </button>
          {excalidrawEl}
        </div>
      </div>
      {/* Render modal via Portal directly into document.body */}
      {mounted && modalContent && createPortal(modalContent, document.body)}
    </>
  );
}
