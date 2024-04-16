use std::{
    default,
    ffi::{c_void, CStr},
};

use crate::core::private::internal_ReactorAPI;
use crate::core::*;
use crate::sys;

pub struct ObserverBuilder<'a, T>
where
    T: Iterable,
{
    desc: sys::ecs_observer_desc_t,
    term_builder: TermBuilder,
    world: WorldRef<'a>,
    event_count: i32,
    is_instanced: bool,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> ObserverBuilder<'a, T>
where
    T: Iterable,
{
    /// Create a new observer builder
    ///
    /// # Arguments
    ///
    /// * `world` - The world to create the observer in
    ///
    /// See also
    ///
    /// * C++ API: `observer_builder::observer_builder`
    #[doc(alias = "observer_builder::observer_builder")]
    pub fn new(world: impl IntoWorld<'a>) -> Self {
        let desc = Default::default();
        let mut obj = Self {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 0,
            is_instanced: false,
            world: world.world(),
            _phantom: std::marker::PhantomData,
        };

        obj.desc.entity =
            unsafe { sys::ecs_entity_init(world.world_ptr_mut(), &Default::default()) };

        T::populate(&mut obj);
        obj
    }

    /// Create a new observer builder with a name
    ///
    /// # Arguments
    ///
    /// * `world` - The world to create the observer in
    /// * `name` - The name of the observer
    ///
    /// See also
    ///
    /// * C++ API: `node_builder::node_builder`
    #[doc(alias = "node_builder::node_builder")]
    pub fn new_named(world: impl IntoWorld<'a>, name: &CStr) -> Self {
        let desc = Default::default();
        let mut obj = Self {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 0,
            is_instanced: false,
            world: world.world(),
            _phantom: std::marker::PhantomData,
        };
        let entity_desc: sys::ecs_entity_desc_t = sys::ecs_entity_desc_t {
            name: name.as_ptr(),
            sep: SEPARATOR.as_ptr(),
            root_sep: SEPARATOR.as_ptr(),
            ..default::Default::default()
        };

        obj.desc.entity = unsafe { sys::ecs_entity_init(obj.world_ptr_mut(), &entity_desc) };

        T::populate(&mut obj);
        obj
    }

    /// Create a new observer builder from an existing descriptor
    ///
    /// # Arguments
    ///
    /// * `world` - The world to create the observer in
    /// * `desc` - The descriptor to create the observer from
    ///
    /// See also
    ///
    /// * C++ API: `observer_builder::observer_builder`
    #[doc(alias = "observer_builder::observer_builder")]
    pub(crate) fn new_from_desc(world: impl IntoWorld<'a>, desc: sys::ecs_observer_desc_t) -> Self {
        let mut obj = Self {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 0,
            world: world.world(),
            is_instanced: false,
            _phantom: std::marker::PhantomData,
        };

        if obj.desc.entity == 0 {
            obj.desc.entity =
                unsafe { sys::ecs_entity_init(world.world_ptr_mut(), &Default::default()) };
        }

        T::populate(&mut obj);
        obj
    }

    pub fn event_count(&self) -> i32 {
        self.event_count
    }

    /// Specify the event(s) for when the observer should run.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to add
    ///
    /// # See also
    ///
    /// * C++ API: `observer_builder_i::event`
    #[doc(alias = "observer_builder_i::event")]
    pub fn add_event_id(&mut self, event: impl Into<Entity>) -> &mut Self {
        let event = *event.into();
        let event_count = self.event_count as usize;
        self.event_count += 1;
        self.desc.events[event_count] = event;
        self
    }

    /// Specify the event(s) for when the observer should run.
    ///
    /// # Type parameters
    ///
    /// * `T` - The type of the event
    ///
    /// # See also
    ///
    /// * C++ API: `observer_builder_i::event`
    #[doc(alias = "observer_builder_i::event")]
    pub fn add_event<E>(&mut self) -> &mut Self
    where
        E: ComponentId,
    {
        let event_count = self.event_count as usize;
        self.event_count += 1;
        let id = E::get_id(self.world());
        self.desc.events[event_count] = id;
        self
    }

    /// Invoke observer for anything that matches its filter on creation
    ///
    /// # Arguments
    ///
    /// * `should_yield` - If true, the observer will be invoked for all existing entities that match its filter
    ///
    /// # See also
    ///
    /// * C++ API: `observer_builder_i::yield_existing`
    #[doc(alias = "observer_builder_i::yield_existing")]
    pub fn yield_existing(&mut self, should_yield: bool) -> &mut Self {
        self.desc.yield_existing = should_yield;
        self
    }
}

impl<'a, T: Iterable> QueryConfig<'a> for ObserverBuilder<'a, T> {
    fn term_builder(&self) -> &TermBuilder {
        &self.term_builder
    }

    fn term_builder_mut(&mut self) -> &mut TermBuilder {
        &mut self.term_builder
    }

    fn query_desc(&self) -> &sys::ecs_query_desc_t {
        &self.desc.query
    }

    fn query_desc_mut(&mut self) -> &mut sys::ecs_query_desc_t {
        &mut self.desc.query
    }
}
impl<'a, T: Iterable> TermBuilderImpl<'a> for ObserverBuilder<'a, T> {}

impl<'a, T: Iterable> QueryBuilderImpl<'a> for ObserverBuilder<'a, T> {}

impl<'a, T> Builder<'a> for ObserverBuilder<'a, T>
where
    T: Iterable,
{
    type BuiltType = Observer<'a>;

    /// Build the `observer_builder` into an `observer`
    ///
    /// See also
    ///
    /// * C++ API: `node_builder::build`
    #[doc(alias = "node_builder::build")]
    fn build(&mut self) -> Self::BuiltType {
        Observer::new(self.world(), self.desc, self.is_instanced)
    }
}

impl<'a, T: Iterable> IntoWorld<'a> for ObserverBuilder<'a, T> {
    fn world(&self) -> WorldRef<'a> {
        self.world
    }
}

implement_reactor_api!(ObserverBuilder<'a, T>);
