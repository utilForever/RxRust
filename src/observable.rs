use crate::prelude::*;

pub enum OState<E> {
  Next,
  Complete,
  Err(E),
}

pub trait Observable<'a>: Sized {
  /// The type of the elements being emitted.
  type Item: Sized;
  //
  type Err;
  // the Subscription subsribe method return.
  type Unsubscribe: Subscription<'a, Err = Self::Err> + 'a;

  fn subscribe_return_state<N>(self, next: N) -> Self::Unsubscribe
  where
    N: 'a + FnMut(Self::Item) -> OState<Self::Err>;

  fn subscribe<N>(self, mut next: N) -> Self::Unsubscribe
  where
    N: 'a + FnMut(Self::Item),
  {
    self.subscribe_return_state(move |v| {
      next(v);
      OState::Next
    })
  }

  fn broadcast(self) -> Subject<'a, Self::Item, Self::Err>
  where
    Self: 'a,
  {
    Subject::from_stream(self)
  }
}
