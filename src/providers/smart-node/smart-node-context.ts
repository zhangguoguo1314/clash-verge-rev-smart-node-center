import type { SmartNodeConfig, SmartGroup, SmartNode } from '@/types/smart-node'
import { createContext, useContext } from 'react'

export interface SNCContextType {
  enabled: boolean
  pool: SmartNode[] | undefined
  groups: SmartGroup[] | undefined
  config: SmartNodeConfig | undefined
  toggleEnabled: (enabled: boolean) => Promise<void>
  toggling: boolean
  refreshAll: () => Promise<void>
}

export const SNCContext = createContext<SNCContextType | null>(
  null,
)

export const useSNC = (): SNCContextType => {
  const ctx = useContext(SNCContext)
  if (!ctx) {
    throw new Error(
      'useSNC must be used within SncProvider',
    )
  }
  return ctx
}
