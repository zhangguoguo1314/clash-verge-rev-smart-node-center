import { useLockFn } from 'ahooks'

import type { SmartNodeConfig } from '@/types/smart-node'
import { getCacheData, setCacheData, useQuery } from '@/services/query-client'
import {
  sncGetConfig,
  sncPatchConfig,
} from '@/services/smart-node'

export function useSmartNodeConfig() {
  const { data, ...rest } = useQuery({
    queryKey: ['sncGetConfig'],
    queryFn: () => sncGetConfig(),
  })

  const [patchConfig, patching] = useLockFn(
    async (config: Partial<SmartNodeConfig>) => {
      const prev = getCacheData<SmartNodeConfig>([
        'sncGetConfig',
      ])
      const merged = { ...prev, ...config }
      setCacheData(['sncGetConfig'], merged)
      await sncPatchConfig(config)
      rest.mutate()
    },
  )

  return {
    config: data as SmartNodeConfig | undefined,
    patchConfig,
    patching,
    ...rest,
  }
}
