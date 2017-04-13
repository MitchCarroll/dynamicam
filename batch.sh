for f in input/*.cam ; 
    do FILENAME=`basename ${f%%.*}`; 
    ./target/debug/dynamicam input/${FILENAME}.cam output/${FILENAME}.xpm; 
done
