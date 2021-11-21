###
 # @Author: Yinwhe
 # @Date: 2021-11-20 22:30:12
 # @LastEditors: Yinwhe
 # @LastEditTime: 2021-11-21 16:23:46
 # @Description: file information
 # @Copyright: Copyright (c) 2021
### 

javac Test.java
sh build.sh
touch res.txt score
java Test res.txt
cat res.txt score

# clear
rm *.class score res.txt