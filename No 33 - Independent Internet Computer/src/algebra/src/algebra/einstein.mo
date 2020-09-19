actor einstein {
    public func gauss_sum(x:Int,y:Int) : async Int {
        var total:Int=0;
        var counter=x;
        while(counter<=y)
        {
            total+=counter;
            counter+=1;
        };
        return total;
  };
  
};
