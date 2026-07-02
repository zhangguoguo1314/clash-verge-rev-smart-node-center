import React, { useCallback, useMemo } from 'react'

import {
  useGroups,
  usePool,
  useSmartNodeConfig,
  useSmartNodeEnabled,
} from '@/hooks/use-smart-node'
import { SNCContext } from './smart-node-context'

export const SncProvider = ({
  children,
}: {
  children: React.ReactNode
}) => {
  const { enabled, toggle, toggling } =
    useSmartNodeEnabled()

  const { pool, refreshPool } = usePool()
  const { groups, refreshGroups } = useGroups()
  const { config } = useSmartNodeConfig()

  const refreshAll = useCallback(async () => {
    await Promise.all([refreshPool(), refreshGroups()])
  }, [refreshPool, refreshGroups])

  const value = useMemo<SNCContextType>(
    () => ({
      enabled: enabled ?? false,
      pool,
      groups,
      config,
      toggleEnabled: toggle,
      toggling,
      refreshAll,
    }),
    [
      enabled,
      pool,
      groups,
      config,
      toggle,
      toggling,
      refreshAll,
    ],
  )

  if (!enabled) {
    return null
  }

  return (
    <SNCContext value={value}>{children}</SNCContext>
  )
}
