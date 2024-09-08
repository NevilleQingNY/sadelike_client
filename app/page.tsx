'use client'

import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

interface Item {
  id: number;
  content: string;
}

export default function Home() {
  const [items, setItems] = useState<Item[]>([])
  const [newItem, setNewItem] = useState('')

  useEffect(() => {
    loadItems()
  }, [])

  

  const loadItems = async () => {
    try {
      const fetchedItems = await invoke<[number, string][]>('get_items')
      setItems(fetchedItems.map(([id, content]) => ({ id, content })))
    } catch (error) {
      console.error('Failed to load items:', error)
    }
  }

  const addItem = async () => {
    if (newItem.trim()) {
      try {
        await invoke('add_item', { content: newItem })
        setNewItem('')
        await loadItems()
      } catch (error) {
        console.error('Failed to add item:', error)
      }
    }
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1 className="text-4xl font-bold mb-8">SQLite Demo with Tauri</h1>
      <div className="w-full max-w-md space-y-4">
        <div className="flex space-x-2">
          <Input
            type="text"
            value={newItem}
            onChange={(e) => setNewItem(e.target.value)}
            placeholder="Enter new item"
          />
          <Button onClick={addItem}>Add</Button>
        </div>
        <ul className="space-y-2">
          {items.map((item) => (
            <li key={item.id} className="bg-gray-100 p-2 rounded">
              {item.content}
            </li>
          ))}
        </ul>
      </div>
    </main>
  )
}
