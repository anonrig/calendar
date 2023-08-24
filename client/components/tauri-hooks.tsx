'use client'

import { EventCallback, UnlistenFn, listen, once } from '@tauri-apps/api/event'
import { useEffect } from 'react'

export type TauriHookCallback<T> = EventCallback<T>

// Make sure the callback is surrounded with `useCallback`
export function useListen<T>(key: string, callback: TauriHookCallback<T>) {
  useEffect(() => {
    let unlisten: UnlistenFn | undefined

    listen<T>(key, callback)
      .then((handler) => {
        unlisten = handler
      })
      .catch((error) => console.error(error))

    return () => {
      unlisten?.()
    }
  }, [])
}

// Make sure the callback is surrounded with `useCallback`
export function useOnce<T>(key: string, callback: TauriHookCallback<T>) {
  useEffect(() => {
    let unlisten: UnlistenFn | undefined

    once<T>(key, callback)
      .then((handler) => {
        unlisten = handler
      })
      .catch((error) => console.error(error))

    return () => {
      unlisten?.()
    }
  }, [callback])
}
