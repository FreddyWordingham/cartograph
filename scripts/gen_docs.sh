#!/bin/bash

cargo doc;
rm -r docs;
mv target/doc docs;
echo "<meta http-equiv=refresh content=0;url=cartograph/index.html>" > docs/index.html;
