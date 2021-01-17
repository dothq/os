icons=(close depth hide maximize menuButton minimize restore rolldown rollup)

cd icons

rm -rf tmp
mkdir -p tmp
cp *.svg tmp/

exportArgs=""

for i in "${icons[@]}"; do
  fileSvg="tmp/${i}.svg"
  fileXpm="tmp/${i}.xpm"

  fileExport="-b '(let* ((image (car (file-svg-load RUN-NONINTERACTIVE \"${fileSvg}\" \"${fileSvg}\" 0 0 0 1))) (drawable (car (gimp-image-get-active-layer image)))) (file-xpm-save RUN-NONINTERACTIVE image drawable \"${fileXpm}\" \"${fileXpm}\" 255))'"
  exportArgs="${exportArgs} ${fileExport}"
done

gimpCommand="gimp ${exportArgs} -b '(gimp-quit 0)'"
eval $gimpCommand

for i in "${icons[@]}"; do
  rm "../light/${i}A.xpm" "../light/${i}I.xpm"

  mv "tmp/${i}.xpm" "../light/${i}A.xpm"
  cp "../light/${i}A.xpm" "../light/${i}I.xpm"
done

exportArgs=""

for i in "${icons[@]}"; do
  fileSvg="tmp/${i}.svg"
  fileXpm="tmp/${i}.xpm"

  fileExport="-b '(let* ((image (car (file-svg-load RUN-NONINTERACTIVE \"${fileSvg}\" \"${fileSvg}\" 0 0 0 1))) (drawable (car (gimp-image-get-active-layer image)))) (gimp-drawable-invert drawable TRUE) (file-xpm-save RUN-NONINTERACTIVE image drawable \"${fileXpm}\" \"${fileXpm}\" 255))'"
  exportArgs="${exportArgs} ${fileExport}"
done

gimpCommand="gimp ${exportArgs} -b '(gimp-quit 0)'"
eval $gimpCommand

for i in "${icons[@]}"; do
  rm "../dark/${i}A.xpm" "../dark/${i}I.xpm"

  mv "tmp/${i}.xpm" "../dark/${i}A.xpm"
  cp "../dark/${i}A.xpm" "../dark/${i}I.xpm"
done

rm -rf tmp
