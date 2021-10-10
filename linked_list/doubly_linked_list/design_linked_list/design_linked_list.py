'''
Author: Nathan Bockisch
Date: October 9, 2021
'''

class Node:

    def __init__(self, val: int):
        self.val = val
        self.next = None
        self.prev = None

class MyLinkedList:

    def __init__(self):
        self.head = None
        self.len = 0

    def getNodeAtIndex(self, index: int) -> Node:
        # Check for invalid index
        if (self.len <= index):
            return None
        
        cur = self.head
        i = 0
        
        while (cur and i < index):
            cur = cur.next
            i += 1
            
        return cur
    
    def get(self, index: int) -> int:
        cur = self.getNodeAtIndex(index)

        # Check for invalid index
        if (not cur):
            return -1
        
        return cur.val

    def addAtHead(self, val: int) -> None:
        self.addAtIndex(0, val) 

    def addAtTail(self, val: int) -> None:
        self.addAtIndex(self.len, val)

    def addAtIndex(self, index: int, val: int) -> None:
        # Check if adding at head
        if (index == 0):
            new_head = Node(val)
            new_head.next = self.head
            self.head = new_head
            self.len += 1
            return
        
        cur = self.getNodeAtIndex(index - 1)

        # Check for invalid index
        if (not cur):
            return

        # Insert the node at index
        new_node = Node(val)
        new_node.prev = cur
        new_node.next = cur.next
        if (cur.next):
            cur.next.prev = new_node
        cur.next = new_node 
        self.len += 1

    def deleteAtIndex(self, index: int) -> None:
        # Check if deleting at head
        if (index == 0 and self.head):
            self.head = self.head.next
            self.len -= 1
            return
        
        cur = self.getNodeAtIndex(index - 1)
            
        # Check for invalid index
        if (not cur or not cur.next):
            return 
        
        if (cur.next.next):
            cur.next.next.prev = cur 
            
        cur.next = cur.next.next
        self.len -= 1

# Your MyLinkedList object will be instantiated and called as such:
# obj = MyLinkedList()
# param_1 = obj.get(index)
# obj.addAtHead(val)
# obj.addAtTail(val)
# obj.addAtIndex(index,val)
# obj.deleteAtIndex(index)
