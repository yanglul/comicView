pub trait Transport{
    fn send_msg(&self,msg:&str)->String;
    fn download(&self,msg:&str,path:&str);
}