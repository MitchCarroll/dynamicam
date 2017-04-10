for f in input/*.cam ; 
    do FILENAME=`basename ${f%%.*}`; 
    ./dynamicamo input/${FILENAME}.cam output/${FILENAME}.xpm; 
done
