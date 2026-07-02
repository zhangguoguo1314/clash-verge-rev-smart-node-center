import { useLockFn } from 'ahooks'

import { useQuery } from '@/services/query-client'
import {
  sncIsEnabled,
  sncToggleEnabled,
} from '@/services/smart-node'

export function useSmartNodeEnabled() {
  const { data, ...rest } = useQuery({
    queryKey: ['sncIsEnabled'],
    queryFn: () => sncIsEnabled(),
  })

  const [toggle, toggling] = useLockFn(
    async (enabled: boolean) => {
      await sncToggleEnabled(enabled)
      rest.mutate()
    },
  )

  return {
    enabled: data ?? false,
    toggle,
    toggling,
    ...rest,
  }
}
