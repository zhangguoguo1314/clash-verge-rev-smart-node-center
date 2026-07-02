import { useLockFn } from 'ahooks'

import type { GroupDetectResult, SmartGroup } from '@/types/smart-node'
import { revalidateQuery, useQuery } from '@/services/query-client'
import {
  sncCreateGroup,
  sncDeleteGroup,
  sncDetectGroup,
  sncGetGroups,
  sncUpdateGroup,
} from '@/services/smart-node'

export function useGroups() {
  const { data: groups, ...rest } = useQuery({
    queryKey: ['sncGetGroups'],
    queryFn: () => sncGetGroups(),
  })

  const [createGroup, creating] = useLockFn(
    async (group: Partial<SmartGroup>) => {
      await sncCreateGroup(group)
      rest.mutate()
    },
  )

  const [updateGroup, updating] = useLockFn(
    async (id: string, group: Partial<SmartGroup>) => {
      await sncUpdateGroup(id, group)
      rest.mutate()
    },
  )

  const [deleteGroup, deleting] = useLockFn(
    async (id: string) => {
      await sncDeleteGroup(id)
      rest.mutate()
    },
  )

  const [detectGroup, detecting] = useLockFn(
    async (
      groupId: string,
    ): Promise<GroupDetectResult> => {
      const result = await sncDetectGroup(groupId)
      rest.mutate()
      return result
    },
  )

  const refreshGroups = () =>
    revalidateQuery(['sncGetGroups'])

  return {
    groups: groups ?? [],
    createGroup,
    creating,
    updateGroup,
    updating,
    deleteGroup,
    deleting,
    detectGroup,
    detecting,
    refreshGroups,
    ...rest,
  }
}
