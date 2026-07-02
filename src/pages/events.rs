use crate::components::cards::{AllEventCard, FeaturedEventCard};
use crate::components::filters::{FilterGroup, FilterOption, Filters};
use leptos::IntoView;
use leptos::prelude::*;

#[component]
pub fn Events() -> impl IntoView {
    let filter_groups = vec![
        FilterGroup::new(
            "Category:",
            vec![
                FilterOption::new("All", true),
                FilterOption::new("Khutbah", false),
                FilterOption::new("Lecture", false),
                FilterOption::new("Halaqah", false),
                FilterOption::new("Youth", false),
                FilterOption::new("Sisters", false),
                FilterOption::new("Workshop", false),
                FilterOption::new("Career", false),
                FilterOption::new("Charity", false),
            ],
        ),
        FilterGroup::new(
            "Time:",
            vec![
                FilterOption::new("Today", false),
                FilterOption::new("This Week", true),
                FilterOption::new("This Month", false),
                FilterOption::new("Upcoming", false),
            ],
        ),
    ];

    view! {
        <div class="mr-4 space-y-12 py-8">
            <Filters
                search_placeholder="Search events, topics, or mosques".to_string()
                location_label="Detroit, MI".to_string()
                filter_groups=filter_groups
            />

            <section class="space-y-6">
                <div class="flex items-center justify-between">
                    <h1 class="text-3xl font-bold text-foreground-900">"Featured Events"</h1>
                    <a href="#" class="font-semibold text-primary hover:underline">"View all →"</a>
                </div>

                <div class="grid gap-8 lg:grid-cols-2">
                    <FeaturedEventCard
                        index = 0
                        badge="Featured".to_string()
                        title="Jumu'ah: The Prophetic Character".to_string()
                        date="Friday, Dec 15 · 1:00 PM".to_string()
                        location="Islamic Center of Detroit".to_string()
                        description="Join us for this week's Jumu'ah khutbah focusing on the noble character of Prophet Muhammad ﷺ and how we can embody these teachings in our daily lives. Sheikh Ahmed will lead the congregation.".to_string()
                        cta_label="View Details".to_string()
                    />

                    <FeaturedEventCard
                        index = 1
                        badge="Eid Event".to_string()
                        title="Eid al-Adha Celebration".to_string()
                        date="Saturday, Dec 16 · 8:00 AM".to_string()
                        location="Dearborn Masjid".to_string()
                        description="Community-wide Eid prayers followed by breakfast and activities for the whole family. Children's activities, communal meal, and opportunities to connect with fellow community members.".to_string()
                        cta_label="RSVP Now".to_string()
                    />
                </div>
            </section>

            <section class="space-y-6">
                <h1 class="text-3xl font-bold text-foreground-900">"All Events"</h1>

                <div class="grid gap-8 md:grid-cols-2 xl:grid-cols-3">
                    <AllEventCard
                        index=0
                        category="Lecture".to_string()
                        distance="2.3 mi".to_string()
                        title="Understanding Surah Al-Kahf".to_string()
                        mosque="Masjid Al-Noor".to_string()
                        date="Sat, Dec 16 · 10:00 AM".to_string()
                        description="Deep dive into the stories and lessons of Surah Al-Kahf with Sheikh Omar. Weekly series covering practical reflections and timeless guidance.".to_string()
                        cta_label="View Details".to_string()
                    />

                    <AllEventCard
                        index=1
                        category="Workshop".to_string()
                        distance="4.1 mi".to_string()
                        title="Resume Building Workshop".to_string()
                        mosque="Islamic Community Center".to_string()
                        date="Sun, Dec 17 · 2:00 PM".to_string()
                        description="Professional career workshop helping young professionals craft compelling resumes and prepare for upcoming interviews.".to_string()
                        cta_label="Register".to_string()
                    />

                    <AllEventCard
                        index=2
                        category="Halaqah".to_string()
                        distance="1.8 mi".to_string()
                        title="Youth Halaqah: Living Faith".to_string()
                        mosque="Detroit Central Mosque".to_string()
                        date="Sun, Dec 17 · 6:30 PM".to_string()
                        description="Interactive discussion circle for young adults exploring how to live faith authentically in everyday life.".to_string()
                        cta_label="Join".to_string()
                    />

                    <AllEventCard
                        index=3
                        category="Sisters".to_string()
                        distance="3.2 mi".to_string()
                        title="Sisters' Study Circle".to_string()
                        mosque="Masjid Bilal".to_string()
                        date="Mon, Dec 18 · 7:30 PM".to_string()
                        description="Monthly gathering for sisters to study Islamic history, discuss contemporary issues, and build community.".to_string()
                        cta_label="RSVP".to_string()
                    />

                    <AllEventCard
                        index=4
                        category="Charity".to_string()
                        distance="5.6 mi".to_string()
                        title="Food Drive Volunteer Day".to_string()
                        mosque="Merzah Community Outreach".to_string()
                        date="Sat, Dec 23 · 9:00 AM".to_string()
                        description="Help organize and distribute food packages to families in need. All ages welcome. Light refreshments provided.".to_string()
                        cta_label="Volunteer".to_string()
                    />

                    <AllEventCard
                        index=5
                        category="Family".to_string()
                        distance="2.9 mi".to_string()
                        title="Family Game Night".to_string()
                        mosque="Islamic Center of Detroit".to_string()
                        date="Fri, Dec 22 · 6:00 PM".to_string()
                        description="Bring the whole family for an evening of board games, activities, and fellowship. Dinner and snacks included.".to_string()
                        cta_label="Register Family".to_string()
                    />
                </div>
            </section>
        </div>
    }
}
