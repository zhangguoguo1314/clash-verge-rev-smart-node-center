import {
  Box,
  Skeleton,
  Typography,
  alpha,
  useTheme,
} from '@mui/material'
import { PublicRounded } from '@mui/icons-material'
import { useTranslation } from 'react-i18next'

import { EnhancedCard } from '@/components/home/enhanced-card'
import { useDashboard, useIpInfo } from '@/hooks/use-smart-node'

export const InfoCard = () => {
  const { t } = useTranslation('smartNode')
  const theme = useTheme()
  const { dashboardData, isPending } = useDashboard()
  const { ipInfo } = useIpInfo()

  const isLoading = isPending

  return (
    <EnhancedCard
      title={t('dashboard.ipInfo')}
      icon={<PublicRounded />}
      iconColor="primary"
    >
      {isLoading ? (
        <Box>
          <Skeleton width="60%" height={28} />
          <Skeleton width="80%" height={20} />
          <Skeleton width="70%" height={20} />
        </Box>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
          <Typography variant="body2" color="text.secondary">
            {t('dashboard.currentNode')}
          </Typography>
          <Typography variant="h6" fontWeight="bold">
            {dashboardData?.current_node ?? '-'}
          </Typography>
          <Box
            sx={{
              mt: 1,
              p: 1.5,
              borderRadius: 1.5,
              bgcolor: alpha(
                theme.palette.primary.main,
                0.06,
              ),
            }}
          >
            <Typography variant="caption" color="text.secondary">
              IP: {ipInfo?.ip ?? '-'}
            </Typography>
            <Typography variant="caption" color="text.secondary" display="block">
              {ipInfo?.country ?? '-'}
              {ipInfo?.city ? ` / ${ipInfo.city}` : ''}
            </Typography>
            <Typography variant="caption" color="text.secondary">
              {ipInfo?.isp ?? '-'}
            </Typography>
          </Box>
          {dashboardData?.current_latency != null && (
            <Typography variant="body2">
              {t('dashboard.latency')}: {dashboardData.current_latency} ms
            </Typography>
          )}
        </Box>
      )}
    </EnhancedCard>
  )
}
