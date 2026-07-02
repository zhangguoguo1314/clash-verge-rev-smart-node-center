import {
  Box,
  LinearProgress,
  Skeleton,
  Typography,
  alpha,
  useTheme,
} from '@mui/material'
import { ComputerRounded } from '@mui/icons-material'
import { useTranslation } from 'react-i18next'

import { EnhancedCard } from '@/components/home/enhanced-card'
import { useDashboard } from '@/hooks/use-smart-node'

const formatUptime = (minutes: number): string => {
  const days = Math.floor(minutes / 1440)
  const hours = Math.floor((minutes % 1440) / 60)
  if (days > 0) return `${days}d ${hours}h`
  return `${hours}h`
}

export const SystemCard = () => {
  const { t } = useTranslation('smartNode')
  const theme = useTheme()
  const { dashboardData, isPending } = useDashboard()

  const isLoading = isPending

  return (
    <EnhancedCard
      title={t('dashboard.system')}
      icon={<ComputerRounded />}
      iconColor="info"
    >
      {isLoading ? (
        <Box>
          <Skeleton width="50%" height={24} />
          <Skeleton width="60%" height={24} />
          <Skeleton width="40%" height={24} />
        </Box>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1.5 }}>
          <Box>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 0.5 }}>
              <Typography variant="body2" color="text.secondary">
                CPU
              </Typography>
              <Typography variant="body2" fontWeight="bold">
                {dashboardData?.cpu_usage?.toFixed(1) ?? '0.0'}%
              </Typography>
            </Box>
            <LinearProgress
              variant="determinate"
              value={dashboardData?.cpu_usage ?? 0}
              sx={{
                height: 6,
                borderRadius: 3,
                bgcolor: alpha(
                  theme.palette.info.main,
                  0.1,
                ),
              }}
            />
          </Box>
          <Box>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 0.5 }}>
              <Typography variant="body2" color="text.secondary">
                {t('dashboard.memory')}
              </Typography>
              <Typography variant="body2" fontWeight="bold">
                {dashboardData?.memory_usage?.toFixed(1) ?? '0.0'}%
              </Typography>
            </Box>
            <LinearProgress
              variant="determinate"
              value={dashboardData?.memory_usage ?? 0}
              sx={{
                height: 6,
                borderRadius: 3,
                bgcolor: alpha(
                  theme.palette.warning.main,
                  0.1,
                ),
              }}
            />
          </Box>
          <Box
            sx={{
              mt: 1,
              display: 'flex',
              justifyContent: 'space-between',
            }}
          >
            <Typography variant="caption" color="text.secondary">
              {t('dashboard.nodes')}: {dashboardData?.node_count ?? 0}
            </Typography>
            <Typography variant="caption" color="text.secondary">
              {t('dashboard.avgScore')}: {dashboardData?.avg_score?.toFixed(0) ?? '-'}
            </Typography>
            <Typography variant="caption" color="text.secondary">
              {t('dashboard.uptime')}: {formatUptime(dashboardData?.uptime_minutes ?? 0)}
            </Typography>
          </Box>
          <Box
            sx={{
              p: 1.5,
              borderRadius: 1.5,
              bgcolor: alpha(
                theme.palette.success.main,
                0.06,
              ),
              textAlign: 'center',
            }}
          >
            <Typography variant="h6" fontWeight="bold" color="success.main">
              {t('dashboard.availability')}: {(dashboardData?.availability_rate ?? 0) * 100}%
            </Typography>
          </Box>
        </Box>
      )}
    </EnhancedCard>
  )
}
