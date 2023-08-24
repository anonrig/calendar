'use client'

import { TauriHookCallback, useListen } from '@/components/tauri-hooks'
import { PropsWithChildren, useCallback } from 'react'

export default function Providers({ children }: PropsWithChildren) {
  const deeplinkHandler: TauriHookCallback<string> = useCallback((event) => {
    console.log('deeplink event received', event)
  }, [])

  // Listen for deep-link events
  useListen('deeplink-received', deeplinkHandler)

  return children
}
