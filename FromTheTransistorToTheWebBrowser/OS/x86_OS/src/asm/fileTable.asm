;;; 
;;; Basic 'file table' made with db. Strings consist of '{fileName1-sector#, fileName2-sector#, ... fileNameN-sector#}'
;;; 

db '{calculator-04,program2-06}'

  ;; sector padding magic!
  times 512-($-$$) db 0         ;; pad rest of file to 0s till 512th byte/end of sector
