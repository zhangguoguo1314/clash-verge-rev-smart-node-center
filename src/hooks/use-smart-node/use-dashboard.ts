import type { DashboardData } from '@/types/smart-node'
import { useQuery } from '@/services/query-client'
import {
  sncCheckCustomUrl,
  sncCheckMediaUnlock,
  sncGetDashboardData,
  sncGetIpInfo,
} from '@/services/smart-node'

export function useDashboard() {
  const { data, ...rest } = useQuery({
    queryKey: ['sncGetDashboardData'],
    queryFn: () => sncGetDashboardData(),
    refetchInterval: 10_000,
  })

  return {
    dashboardData: data as DashboardData | undefined,
    ...rest,
  }
}

export function useIpInfo() {
  const { data, ...rest } = useQuery({
    queryKey: ['sncGetIpInfo'],
    queryFn: () => sncGetIpInfo(),
  })

  return {
    ipInfo: data,
    ...rest,
  }
}

export function useMediaUnlock() {
  const { data, ...rest } = useQuery({
    queryKey: ['sncCheckMediaUnlock'],
    queryFn: () => sncCheckMediaUnlock(),
  })

  return {
    mediaUnlock: data,
    ...rest,
  }
}

export function useCustomUrlCheck() {
  const checkUrl = async (url: string) => {
    return sncCheckCustomUrl(url)
  }

  return { checkUrl }
}
