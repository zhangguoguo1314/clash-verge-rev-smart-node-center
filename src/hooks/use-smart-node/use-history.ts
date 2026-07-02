import type { HistoryRecord, NodeStats } from '@/types/smart-node'
import { useLockFn } from 'ahooks'
import { useQuery } from '@/services/query-client'
import {
  sncCleanupHistory,
  sncGetHistory,
  sncGetStats,
} from '@/services/smart-node'

export function useHistory(nodeUid: string, days: number = 7) {
  const { data: history, ...rest } = useQuery({
    queryKey: ['sncGetHistory', nodeUid, days],
    queryFn: () => sncGetHistory(nodeUid, days),
    enabled: !!nodeUid,
  })

  const { data: stats, ...statsRest } = useQuery({
    queryKey: ['sncGetStats', nodeUid, days],
    queryFn: () => sncGetStats(nodeUid, days),
    enabled: !!nodeUid,
  })

  const [cleanup, cleaning] = useLockFn(async () => {
    await sncCleanupHistory()
    rest.mutate()
    statsRest.mutate()
  })

  return {
    history: history ?? [],
    stats: stats as NodeStats | undefined,
    cleanup,
    cleaning,
    ...rest,
  }
}
