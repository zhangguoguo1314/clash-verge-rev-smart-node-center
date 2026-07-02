import { useState } from 'react'
import { useLockFn } from 'ahooks'

import { useQuery } from '@/services/query-client'
import {
  sncGetHealthStatus,
  sncStartHealthCheck,
} from '@/services/smart-node'

export function useHealthCheck() {
  const [checking, setChecking] = useState(false)

  const { data: healthStatus, ...rest } = useQuery({
    queryKey: ['sncGetHealthStatus'],
    queryFn: () => sncGetHealthStatus(),
    refetchInterval: checking ? 3000 : 0,
  })

  const [startCheck, startingCheck] = useLockFn(
    async (nodeUids?: string[]) => {
      setChecking(true)
      await sncStartHealthCheck(nodeUids)
    },
  )

  return {
    healthStatus,
    checking,
    startCheck,
    startingCheck,
    ...rest,
  }
}
