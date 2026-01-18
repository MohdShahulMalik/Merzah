import { defineSchema, string, geometry, datetime, index } from 'smig';

export const mosques = defineSchema({
  table: 'mosques',
  fields: {
    name: string(),
    location: geometry(),
    street: string(),
    city: string(),
    
    adhan_times: {
        fajr: string(),
        dhuhr: string(),
        asr: string(),
        maghrib: string(),
        isha: string(),
        jummah: string(),
    },

    jamat_times: {
        fajr: string(),
        dhuhr: string(),
        asr: string(),
        maghrib: string(),
        isha: string(),
        jummah: string(),
    },

    created_at: datetime().default('time::now()'),
    updated_at: datetime().default('time::now()'),
  },
  indexes: {
    mosque_location_idx: index(['location']),
    idx_mosque_name: index(['name']),
    idx_mosque_city: index(['city']),
  }
});
