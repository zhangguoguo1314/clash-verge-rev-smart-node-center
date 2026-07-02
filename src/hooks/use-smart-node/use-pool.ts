import { useLockFn } from 'ahooks'

import type { ImportResult, PoolStats, SmartNode } from '@/types/smart-node'
import { revalidateQuery, useQuery } from '@/services/query-client'
import {
  sncAddNodes,
  sncExportNodes,
  sncGetPool,
  sncGetPoolStats,
  sncImportNodes,
  sncRemoveNode,
  sncUpdateNode,
} from '@/services/smart-node'

export function usePool() {
  const { data: pool, ...rest } = useQuery({
    queryKey: ['sncGetPool'],
    queryFn: () => sncGetPool(),
  })

  const { data: poolStats, refetch: refetchPoolStats } =
    useQuery({
      queryKey: ['sncGetPoolStats'],
      queryFn: () => sncGetPoolStats(),
    })

  const [addNodes, adding] = useLockFn(
    async (
      nodes: SmartNode[],
      sourceName: string,
    ) => {
      const result = await sncAddNodes(nodes, sourceName)
      rest.mutate()
      refetchPoolStats()
      return result
    },
  )

  const [importNodes, importing] = useLockFn(
    async (
      content: string,
      format: string,
      sourceName: string,
    ) => {
      const result = await sncImportNodes(
        content,
        format,
        sourceName,
      )
      rest.mutate()
      refetchPoolStats()
      return result
    },
  )

  const [removeNode, removing] = useLockFn(
    async (uid: string) => {
      await sncRemoveNode(uid)
      rest.mutate()
      refetchPoolStats()
    },
  )

  const [updateNode, updating] = useLockFn(
    async (uid: string, update: Partial<SmartNode>) => {
      await sncUpdateNode(uid, update)
      rest.mutate()
    },
  )

  const [exportNodes, exporting] = useLockFn(
    async (
      nodeUids: string[],
      format: string,
      outputPath: string,
    ) => {
      await sncExportNodes(nodeUids, format, outputPath)
    },
  )

  const refreshPool = () => revalidateQuery(['sncGetPool'])

  return {
    pool,
    poolStats,
    addNodes,
    adding,
    importNodes,
    importing,
    removeNode,
    removing,
    updateNode,
    updating,
    exportNodes,
    exporting,
    refreshPool,
    ...rest,
  }
}
