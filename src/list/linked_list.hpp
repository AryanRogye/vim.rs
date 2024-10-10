#ifndef __LINKED_LIST_HPP__
#define __LINKED_LIST_HPP__

template<typename T>
class Node{
	public:
		T data;
		Node* prev;
		Node* next;
		Node(T value) : data(value), next(nullptr), prev(nullptr) {}
};

template<typename T>
class LinkedList{
	private:
		Node<T>* head;
		Node<T>* tail;
	public:
		LinkedList() : head(nullptr), tail(nullptr) {}
		void add_data(int index, T data);	// Add data at index
		void remove_data(int index);	// remove given index
		void remove_data();			// remove last index
		T get_data(int index);		// get_data
};

#endif
