#ifndef __LINKED_LIST_H__
#define __LINKED_LIST_H__

#include <cstdint>
template<typename T>
class LinkedList{
	private:
		struct Node{
			T data;
			Node* prev;
			Node* next;
		};

		Node head;
		Node tail;
	public:
		LinkedList() : head(nullptr), tail(nullptr) {}
		void add_data(uint16_t index, T data);
		void remove_data(int index);	// remove given index
		void remove_data();			// remove last index
		T get_data(int index);		// get_data
};

#endif
