import { useState } from 'react'
import { useLockFn } from 'ahooks'

import type { SpeedTestResult } from '@/types/smart-node'
import { useQuery } from '@/services/query-client'
import {
  sncGetSpeedResults,
  sncStartSpeedTest,
  sncStopSpeedTest,
} from '@/services/smart-node'

export function useSpeedTest() {
  const [testing, setTesting] = useState(false)

  const { data: results, ...rest } = useQuery({
    queryKey: ['sncGetSpeedResults'],
    queryFn: () => sncGetSpeedResults(),
    refetchInterval: testing ? 2000 : 0,
  })

  const [startTest, starting] = useLockFn(
    async (nodeUids: string[], testTypes: string[]) => {
      setTesting(true)
      await sncStartSpeedTest(nodeUids, testTypes)
    },
  )

  const [stopTest, stopping] = useLockFn(async () => {
    await sncStopSpeedTest()
    setTesting(false)
    rest.mutate()
  })

  const isRunning = testing && !rest.isPending

  return {
    results: results ?? [],
    testing: isRunning,
    startTest,
    starting,
    stopTest,
    stopping,
    ...rest,
  }
}
