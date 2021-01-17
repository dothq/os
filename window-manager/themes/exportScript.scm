(
  let* (
    (image (car (file-svg-load RUN-NONINTERACTIVE filename filename 0 0 0 1)))
    (drawable (car (gimp-image-get-active-layer image)))
  ) 
  
  (file-xpm-save RUN-NONINTERACTIVE image drawable filename filename 255)
)

(let* ((image (car (file-svg-load RUN-NONINTERACTIVE "tmp/close.svg" "tmp/close.svg" 0 0 0 1))) (drawable (car (gimp-image-get-active-layer image)))) (file-xpm-save RUN-NONINTERACTIVE image drawable "tmp/close.xpm" "tmp/close.xpm" 255))


