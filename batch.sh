for f in input/*.cam ; 
    do FILENAME=`basename ${f%%.*}`; 
    ./target/release/dynamicam input/${FILENAME}.cam output/${FILENAME}.xpm; 
done
