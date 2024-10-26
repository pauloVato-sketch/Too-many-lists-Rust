use std::mem;

pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;
struct Node<T>{
    elem: T,
    next: Link<T>
}



impl<T> List<T>{
    
    pub fn new()-> Self{
        List{ head: None }
    }
    
    pub fn push(&mut self, elem: T){
        let novo_no = Box::new (Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(novo_no)
    }
    
    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map( |node| {
            self.head = node.next;
            node.elem
        })
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = self.head.take();
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