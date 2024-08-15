# 21. Merge Sorted List
# link: https://leetcode.com/problems/merge-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        list3 = ListNode()
        current = list3

        while list1 != None and list2 != None:
            if list1.val < list2.val:
                current.next = list1
                list1 = list1.next
            else:
                current.next = list2
                list2 = list2.next
            current = current.next
        
        while list1 != None:
            current.next = list1
            current = current.next
            list1 = list1.next
        
        while list2 != None:
            current.next = list2
            current = current.next
            list2 = list2.next
            
        return list3.next
    


def create_linked_list(elements):
    if len(elements) == 0:
        return
    head = ListNode(elements[0])
    current = head
    for element in elements[1:]:
        current.next = ListNode(element)
        current = current.next
    return head

def print_linked_list(head):
    current = head
    while current:
        print(current.val, end=" -> ")
        current = current.next
    print("None")

def main():
    list1 = create_linked_list([1, 2, 4])
    list2 = create_linked_list([1, 3, 4])

    solution = Solution()
    merged_list = solution.mergeTwoLists(list1, list2)
    print_linked_list(merged_list)

    list1 = create_linked_list([])
    list2 = create_linked_list([])

    solution = Solution()
    merged_list = solution.mergeTwoLists(list1, list2)
    print_linked_list(merged_list)

    list1 = create_linked_list([])
    list2 = create_linked_list([0])

    solution = Solution()
    merged_list = solution.mergeTwoLists(list1, list2)
    print_linked_list(merged_list)


if __name__ == "__main__":
    main()