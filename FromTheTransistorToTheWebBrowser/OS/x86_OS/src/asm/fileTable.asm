;;; 
;;; Basic 'file table' made with db. Strings consist of '{fileName1-sector#, fileName2-sector#, ... fileNameN-sector#}'
;;; 

db 'bootSect  ','bin',0h,1h,1h,\
   'kernel    ','bin',0h,2h,3h,\
   'fileTable ','txt',0h,5h,1h,\
   'calculator','bin',0h,6h,1h

  ;; sector padding magic!
  times 512-($-$$) db 0         ;; pad rest of file to 0s till 512th byte/end of sector
