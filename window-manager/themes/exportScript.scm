(
  (let* ((filelist (cadr (file-glob "tmp/*.svg" 1))))

  (while (not (null? filelist))
  (
    let* (
      (filename (car filelist))
      (image (car (file-svg-load RUN-NONINTERACTIVE filename filename 0 0 0 1)))
      (drawable (car (gimp-image-get-active-layer image)))
    ) 
    
    (file-xpm-save RUN-NONINTERACTIVE image drawable filename filename 255)
  ))
)

