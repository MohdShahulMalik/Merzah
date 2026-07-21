use crate::components::cards::{ContinueLearningCard, CourseCard, LearningPathCard};
use crate::components::filters::{FilterGroup, FilterOption, Filters};
use leptos::prelude::*;

#[component]
pub fn Learn() -> impl IntoView {
    let filter_groups = vec![
        FilterGroup::new(
            "Category:",
            vec![
                FilterOption::new("All", true),
                FilterOption::new("Faith & Worship", false),
                FilterOption::new("Quran", false),
                FilterOption::new("Seerah", false),
                FilterOption::new("Character", false),
                FilterOption::new("Family", false),
                FilterOption::new("Finance", false),
                FilterOption::new("Career", false),
                FilterOption::new("Study Skills", false),
                FilterOption::new("Technology", false),
                FilterOption::new("Community", false),
            ],
        ),
        FilterGroup::new(
            "Difficulty:",
            vec![
                FilterOption::new("All", true),
                FilterOption::new("Beginner", false),
                FilterOption::new("Intermediate", false),
                FilterOption::new("Advanced", false),
            ],
        ),
    ];

    view! {
        <div class="mr-4 space-y-12 py-8">
            <Filters
                search_placeholder="Search courses, lessons, topics, and educators...".to_string()
                filter_groups=filter_groups
            />

            <section>
                <ContinueLearningCard
                    status="In Progress".to_string()
                    title="Fundamentals of Islamic Finance".to_string()
                    description="Learn the principles of halal investing, riba, and ethical wealth building".to_string()
                    time_remaining="45 min remaining".to_string()
                    lesson_progress="Lesson 7 of 12".to_string()
                    progress_percent=58
                    cta_label="Continue Learning".to_string()
                    img_link="https://images.unsplash.com/photo-1501504905252-473c47e087f8?w=128&h=128&fit=crop".to_string()
                />
            </section>

            <section class="space-y-6">
                <div class="flex items-center justify-between gap-4">
                    <h1 class="text-3xl font-bold text-purple-900">"Featured Courses"</h1>
                    <a href="#" class="font-medium text-purple-600 hover:text-purple-700">
                        "View All →"
                    </a>
                </div>

                <div class="grid gap-6 lg:grid-cols-2 xl:grid-cols-3">
                    <CourseCard
                        category="Faith & Worship".to_string()
                        category_class="bg-green-100 text-green-700".to_string()
                        badge="Beginner".to_string()
                        badge_class="bg-purple-100 text-purple-700".to_string()
                        title="Pillars of Islam".to_string()
                        description="Comprehensive guide to the five pillars: Shahada, Salah, Zakat, Sawm, and Hajj".to_string()
                        duration="6 hours".to_string()
                        lesson_count="15 lessons".to_string()
                        instructor_initials="SK".to_string()
                        instructor_name="Sheikh Khalid Ahmed".to_string()
                        cta_label="Start Course".to_string()
                    />

                    <CourseCard
                        category="Quran".to_string()
                        category_class="bg-blue-100 text-blue-700".to_string()
                        badge="Intermediate".to_string()
                        badge_class="bg-purple-100 text-purple-700".to_string()
                        title="Tajweed Mastery".to_string()
                        description="Perfect your Quranic recitation with proper pronunciation and rules of tajweed".to_string()
                        duration="8 hours".to_string()
                        lesson_count="20 lessons".to_string()
                        progress_percent=35
                        instructor_initials="QF".to_string()
                        instructor_name="Qari Fatima Hassan".to_string()
                        cta_label="Continue Course".to_string()
                    />

                    <CourseCard
                        category="Career".to_string()
                        category_class="bg-amber-100 text-amber-700".to_string()
                        badge="All Levels".to_string()
                        badge_class="bg-purple-100 text-purple-700".to_string()
                        title="Professional Communication".to_string()
                        description="Master effective communication skills grounded in Islamic ethics and adab".to_string()
                        duration="4 hours".to_string()
                        lesson_count="10 lessons".to_string()
                        instructor_initials="AA".to_string()
                        instructor_name="Dr. Aisha Ali".to_string()
                        cta_label="Start Course".to_string()
                    />
                </div>
            </section>

            <section class="space-y-6">
                <h1 class="text-3xl font-bold text-purple-900">"Guided Learning Paths"</h1>

                <div class="grid gap-6 xl:grid-cols-2">
                    <LearningPathCard
                        title="Foundations of Faith".to_string()
                        description="Build strong aqeedah and understanding of core Islamic beliefs".to_string()
                        title_class="text-purple-900".to_string()
                        description_class="text-purple-500".to_string()
                        icon="💡".to_string()
                        icon_class="text-purple-200".to_string()
                        course_count="8 courses".to_string()
                        duration="24 hours".to_string()
                        progress_label="Not Started".to_string()
                        progress_percent=0
                        cta_label="Begin Path".to_string()
                    />

                    <LearningPathCard
                        title="Life Skills".to_string()
                        description="Practical wisdom for family, finance, and personal development".to_string()
                        title_class="text-indigo-900".to_string()
                        description_class="text-indigo-500".to_string()
                        icon="🎚️".to_string()
                        icon_class="text-indigo-200".to_string()
                        course_count="12 courses".to_string()
                        duration="36 hours".to_string()
                        progress_label="25%".to_string()
                        progress_percent=25
                        cta_label="Continue Path".to_string()
                    />

                    <LearningPathCard
                        title="Worship Essentials".to_string()
                        description="Master the rituals of salah, fasting, charity, and more".to_string()
                        title_class="text-teal-900".to_string()
                        description_class="text-teal-500".to_string()
                        icon="☪️".to_string()
                        icon_class="text-teal-200".to_string()
                        course_count="6 courses".to_string()
                        duration="18 hours".to_string()
                        progress_label="Not Started".to_string()
                        progress_percent=0
                        cta_label="Begin Path".to_string()
                    />

                    <LearningPathCard
                        title="Career Growth".to_string()
                        description="Professional development with Islamic ethics and leadership".to_string()
                        title_class="text-amber-900".to_string()
                        description_class="text-amber-500".to_string()
                        icon="⚡".to_string()
                        icon_class="text-amber-200".to_string()
                        course_count="10 courses".to_string()
                        duration="30 hours".to_string()
                        progress_label="12%".to_string()
                        progress_percent=12
                        cta_label="Continue Path".to_string()
                    />
                </div>
            </section>

            <section class="space-y-6">
                <h1 class="text-3xl font-bold text-purple-900">"Recommended for You"</h1>

                <div class="grid gap-6 lg:grid-cols-2 xl:grid-cols-3">
                    <CourseCard
                        category="Character".to_string()
                        category_class="bg-rose-100 text-rose-700".to_string()
                        badge="📍 From your mosque".to_string()
                        badge_class="bg-purple-50 text-purple-600".to_string()
                        title="Building Patience (Sabr)".to_string()
                        description="Develop resilience and patience through Quranic teachings and prophetic guidance".to_string()
                        duration="3 hours".to_string()
                        lesson_count="8 lessons".to_string()
                        instructor_initials="IM".to_string()
                        instructor_name="Imam Muhammad Hassan".to_string()
                        cta_label="View Course".to_string()
                    />

                    <CourseCard
                        category="Study Skills".to_string()
                        category_class="bg-emerald-100 text-emerald-700".to_string()
                        badge="Beginner".to_string()
                        badge_class="bg-purple-100 text-purple-700".to_string()
                        title="Effective Study Habits".to_string()
                        description="Learn time management, focus techniques, and productive study methods from an Islamic perspective".to_string()
                        duration="5 hours".to_string()
                        lesson_count="12 lessons".to_string()
                        instructor_initials="YR".to_string()
                        instructor_name="Yasmin Rahman".to_string()
                        cta_label="Start Course".to_string()
                    />

                    <CourseCard
                        category="Seerah".to_string()
                        category_class="bg-orange-100 text-orange-700".to_string()
                        badge="Intermediate".to_string()
                        badge_class="bg-purple-100 text-purple-700".to_string()
                        title="Life of the Prophet ﷺ".to_string()
                        description="Comprehensive study of the Prophet's life, character, and timeless lessons".to_string()
                        duration="12 hours".to_string()
                        lesson_count="25 lessons".to_string()
                        instructor_initials="OH".to_string()
                        instructor_name="Dr. Omar Hasan".to_string()
                        cta_label="Start Course".to_string()
                    />
                </div>
            </section>
        </div>
    }
}
