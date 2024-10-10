#include "linked_list.h"

template<typename T>
void LinkedList<T>::add_data(uint16_t index, T data) {
	Node* 		newNode = new Node;	// Allocate space
	newNode -> data = data;
	// If empty list
	if(this->head == nullptr){
		this -> head = newNode;
		newNode -> next = nullptr;
		newNode -> prev = nullptr;
		return;
	}
	Node* 		temp 	= this->head;
	// inserting at the head
	if(index == 0)	{

	}
	uint16_t 	count 	= 0;
	while(temp != nullptr) {
		if(count == index){
			newNode->next = temp->data;
			temp->data = newNode->data;
		} else {
			newNode->prev = temp;
		}

		temp = temp->next;
	}
}
template<typename T>
void LinkedList<T>::remove_data(int index){}
template<typename T>
void LinkedList<T>::remove_data() {}
template<typename T>
T LinkedList<T>::get_data(int index) {}
