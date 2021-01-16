cd icons

rm -rf tmp
mkdir -p tmp
cp *.svg tmp/

gimp --verbose --batch='((let* ((filelist (cadr (file-glob "tmp/*.svg" 1)))) (while (not (null? filelist)) (let* ((filename (car filelist)) (image (car (file-svg-load RUN-NONINTERACTIVE filename filename 0 0 0 1))) (drawable (car (gimp-image-get-active-layer image)))) (file-xpm-save RUN-NONINTERACTIVE image drawable filename filename 255))))' -b '(gimp-quit 0)'
