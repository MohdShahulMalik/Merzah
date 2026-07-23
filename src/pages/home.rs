use crate::components::cards::{EducationalResourceCard, MosqueEventCard, NearbyMosqueCard, NextPrayerReminderCard, PrayerCard};
use leptos::IntoView;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (is_current, _) = signal(true);
    let (is_not_current, _) = signal(false);

    view! {
        <div class="space-y-8 mt-4 mr-4 mb-4">
            <NextPrayerReminderCard
                location="Brooklyn, NY".to_string()
                mosque_name="Masjid Al-Farooq".to_string()
                prayer_name="Maghrib".to_string()
                iqamah_time="7:48 PM".to_string()
                hours_remaining="02".to_string()
                minutes_remaining="29".to_string()
                seconds_remaining="23".to_string()
            />

            <div class="flex flex-wrap gap-4">
                <PrayerCard
                    prayer_name="Fajr".to_string()
                    jamat_time="5:30 AM".to_string()
                    adhan_time="5:00 AM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Dhuhr".to_string()
                    jamat_time="1:30 PM".to_string()
                    adhan_time="1:00 PM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Asr".to_string()
                    jamat_time="5:15 PM".to_string()
                    adhan_time="4:45 PM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Maghrib".to_string()
                    jamat_time="7:42 PM".to_string()
                    adhan_time="7:35 PM".to_string()
                    is_current=is_current
                />
                <PrayerCard
                    prayer_name="Isha".to_string()
                    jamat_time="9:15 PM".to_string()
                    adhan_time="8:45 PM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Jumu'ah".to_string()
                    jamat_time="1:30 PM".to_string()
                    adhan_time="1:00 PM".to_string()
                    is_current=is_not_current
                />
            </div>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-[#050047]">"Nearby Mosques"</h2>
                    <a href="#" class="font-medium text-purple-600 hover:text-purple-700">"View All →"</a>
                </div>
                <div class="flex gap-5 overflow-x-scroll pb-4">
                    <NearbyMosqueCard
                        mosque_name="Masjid Al-Farooq".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:48 PM".to_string()
                        distance=0.2
                        is_favorite=true
                    />
                    <NearbyMosqueCard
                        mosque_name="Islamic Center of Brooklyn".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:50 PM".to_string()
                        distance=0.8
                        is_favorite=false
                    />
                    <NearbyMosqueCard
                        mosque_name="Masjid At-Taqwa".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:45 PM".to_string()
                        distance=1.2
                        is_favorite=false
                    />
                    <NearbyMosqueCard
                        mosque_name="Muslim Community Center".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:52 PM".to_string()
                        distance=2.1
                        is_favorite=false
                    />
                </div>
            </section>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-[#050047]">"Upcoming Events"</h2>
                    <a href="#" class="font-medium text-purple-600 hover:text-purple-700">"View All →"</a>
                </div>
                <div class="flex gap-5 overflow-x-scroll pb-4">
                    <MosqueEventCard
                        event_title="Jummah Prayer & Khutbah".to_string()
                        event_type="Khutbah".to_string()
                        event_type_class="bg-[#d9e0ff] text-[#18206d]".to_string()
                        mosque_name="Masjid Al-Farooq".to_string()
                        event_day="Friday".to_string()
                        event_time="1:15 PM".to_string()
                        event_short_description="Weekly Friday sermon focusing on building community ties and strengthening our faith.".to_string()
                    />
                    <MosqueEventCard
                        event_title="Understanding the Quran: Surah Al-Kahf".to_string()
                        event_type="Lecture".to_string()
                        event_type_class="bg-[#7debf0] text-[#064f68]".to_string()
                        mosque_name="Islamic Center of Brooklyn".to_string()
                        event_day="Saturday".to_string()
                        event_time="7:00 PM".to_string()
                        event_short_description="Deep dive into the meanings and lessons from Surah Al-Kahf with Sheikh Abdullah.".to_string()
                    />
                    <MosqueEventCard
                        event_title="Community Breakfast & Youth Program".to_string()
                        event_type="Community".to_string()
                        event_type_class="bg-[#b9f4bd] text-[#11631c]".to_string()
                        mosque_name="Masjid At-Taqwa".to_string()
                        event_day="Sunday".to_string()
                        event_time="11:00 AM".to_string()
                        event_short_description="Join us for a family-friendly breakfast and engaging activities for children and youth.".to_string()
                    />
                </div>
            </section>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-[#050047]">"Recommended"</h2>
                    <a href="#" class="font-medium text-purple-600 hover:text-purple-700">"View All →"</a>
                </div>
                <div class="flex gap-5 overflow-x-scroll pb-4">
                    <EducationalResourceCard
                        icon="📖".to_string()
                        lesson_count="12 lessons".to_string()
                        level="Beginner".to_string()
                        resource_title="Introduction to Tajweed".to_string()
                        resource_by="Sheikh Muhammad Ali".to_string()
                        action_label="Continue Learning".to_string()
                    />
                    <EducationalResourceCard
                        icon="🌙".to_string()
                        lesson_count="8 lessons".to_string()
                        level="All Levels".to_string()
                        resource_title="Ramadan Preparation Guide".to_string()
                        resource_by="Ustadha Fatima Hassan".to_string()
                        action_label="Start Course".to_string()
                    />
                    <EducationalResourceCard
                        icon="🤲".to_string()
                        lesson_count="15 lessons".to_string()
                        level="Intermediate".to_string()
                        resource_title="The Art of Du'a".to_string()
                        resource_by="Imam Yusuf Rahman".to_string()
                        action_label="Continue Learning".to_string()
                    />
                    <EducationalResourceCard
                        icon="📚".to_string()
                        lesson_count="20 lessons".to_string()
                        level="Advanced".to_string()
                        resource_title="Seerah: Life of the Prophet ﷺ".to_string()
                        resource_by="Dr. Omar Suleiman".to_string()
                        action_label="Start Course".to_string()
                    />
                    <EducationalResourceCard
                        icon="💎".to_string()
                        lesson_count="10 lessons".to_string()
                        level="Beginner".to_string()
                        resource_title="Islamic Finance Basics".to_string()
                        resource_by="Sheikh Ahmad Bilal".to_string()
                        action_label="Start Course".to_string()
                    />
                </div>
            </section>
        </div>
    }
}
