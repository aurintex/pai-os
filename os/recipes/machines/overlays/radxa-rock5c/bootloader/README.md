# Radxa Rock 5C U-Boot binaries

Place these files here to have the image build write U-Boot into the image (sector 64 and 16384 for RK3588):

- `idbloader.img`
- `u-boot.itb`

You can obtain them from Radxa's prebuilt images or build from [u-boot](https://github.com/radxa/u-boot) for Rock 5C. If these files are missing, the recipe still builds; the image will not boot until U-Boot is written.
