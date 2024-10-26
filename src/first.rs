use std::mem;

struct Node{
    elem: i32,
    next: Link
}
enum Link{
    Empty,
    More(Box<Node>)
}
pub struct List {
    head: Link,
}


impl List{
    
    pub fn new()-> Self{
        List{ head: Link::Empty }
    }
    
    pub fn push(&mut self, elem: i32){
        let novo_no = Box::new (Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(novo_no)
    }
    
    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            } 
        }
    }
}
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics(){
        let mut list = List::new();
        /// Verifica/Checa se ao tentar retirar elemento
        /// de uma lista vazia, retorna None como deveria.
        assert_eq!(list.pop(), None);
        
        /// Insere items na lista (no momento funciona como pilha)
        list.push(1);
        list.push(2);
        list.push(3);
        
        /// Checa se a remoção funciona
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        
        /// Insere mais items para verificar se não houve corrupção
        list.push(4);
        list.push(5);
        
        /// Checa se a remoção funciona
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        
        /// Checa o restante via exaustão
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}