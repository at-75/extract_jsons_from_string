json='''quygkhbjh{
"kp": "girl",
  "at": "boy"
  "check":{
    "a":"t"
  }
}{"a":"q"}{"s":"x"}ihbljnk'''
a=0
arr=[]
start=0
end=0;
json=json.replace("\n","").replace(" ","")
start=-1
for i in range (len(json)):
    if json[i]=='{' and a==0:
        a+=1
        start=i
    elif json[i] == '{':
        a+=1
    elif json[i]=='}': 
        a-=1  
        if a==0:
            end=i
            arr.append(json[start:end+1])
        
    
    

print(arr)