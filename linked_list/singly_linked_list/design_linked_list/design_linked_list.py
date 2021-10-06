'''
Author: Nathan Bockisch
Date: October 6, 2021
'''

class Node:
    
    def __init__(self, val: int):
        self.val = val
        self.next = None

class MyLinkedList:

    def __init__(self):
        self.head = None
    
    def getNodeAt(self, index: int):
        i = 0
        cur = self.head
        
        while (i < index and cur):
            i += 1
            cur = cur.next
        
        return cur

    def get(self, index: int) -> int:
        node = self.getNodeAt(index)
        
        if (not node):
            return -1
        
        return node.val

    def addAtHead(self, val: int) -> None:
        new_head = Node(val)
        new_head.next = self.head
        self.head = new_head

    def addAtTail(self, val: int) -> None:
        if (not self.head):
            self.addAtHead(val)
            return
        
        cur = self.head
        while (cur.next):
            cur = cur.next
        
        cur.next = Node(val)

    def addAtIndex(self, index: int, val: int) -> None:
        if (index == 0):
            self.addAtHead(val)
            return
        
        prev = self.getNodeAt(index - 1)
        if (prev):
            new_node = Node(val)
            new_node.next = prev.next
            prev.next = new_node

    def deleteAtIndex(self, index: int) -> None:
        if (not self.head):
            return
        
        if (index == 0):
            self.head = self.head.next
            return
        
        prev = self.getNodeAt(index - 1)
        if (prev and prev.next):
            prev.next = prev.next.next

            
# Your MyLinkedList object will be instantiated and called as such:
# obj = MyLinkedList()
# param_1 = obj.get(index)
# obj.addAtHead(val)
# obj.addAtTail(val)
# obj.addAtIndex(index,val)
# obj.deleteAtIndex(index)
