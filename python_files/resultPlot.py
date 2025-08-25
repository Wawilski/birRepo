import matplotlib.pyplot as plt


x_high = list(range(2,11))
x = list(range(2,10))
x_low = list(range(2,9))
x_python_low = list(range(2,8))

#Means 
#Tree Width Rust
#Tree Width Down
twd = [35224,49345,77923,161804,306241,566938,1171366,2364993,5603353]
#Tree Width Up
twu = [30603,49111,87380,184474,387953,759058,1623107,3513744]
#Tree Width Rec
twr =[28552,26684,29198,44219,176837,1056264,7236091,17193570]
#Tree Width DP
twdp =[24396,31099,75171,252817,766993,2462354,7865497]
#Tree Width 
tw =[14799,14293,16560,69365,896419,8761376,184561831]

#Autres invariants comp python Rust

#Variance
rust_var = [22864,16802,14076,12567,13923,14732,16267,17870,19809]
python_var = [109265,63912,31602,20474,18688,21313,24501,26896,32136]
#Proximity/Remoteness
rust_prox = [20958,16109,15483,21612,25196,32319,42360,53905,70827]
python_prox = [108953,62869,33803,24741,25718,31050,42052,52659,68029]
#girth
rust_girth = [19889,15396,15651,17933,23975,30412,39477,49350,62474]
python_girth=[111334,63226,37435,28424,32415,43961,60903,83006,107989]
#tree_width classic


rust_tw =[14799,14293,16560,69365,896419,8761376]
python_tw=[136394,109883,106960,258117,1886869,31683910]





# plt.plot(x_high,twd)
# plt.plot(x,twu)
# plt.plot(x,twr)
# plt.plot(x_low,twdp)
# plt.plot(x_low,tw)

# plt.legend(["Tree Width Rec","Tree Width Brut"])
# plt.legend(["Improved Tree Width Rec Down","Improved Tree Width Rec Up","Tree Width Rec","TWDP"])

#tree_width
# plt.plot(x_python_low,rust_tw)
# plt.plot(x_python_low,python_tw)

# Girth
# plt.plot(x_high,rust_girth)
# plt.plot(x_high,python_girth)

# Var
# plt.plot(x_high,rust_var)
# plt.plot(x_high,python_var)

#Remoteness/Proximity
plt.plot(x_high,rust_prox)
plt.plot(x_high,python_prox)

plt.legend(["Rust","Python"])
plt.show()
#
#
