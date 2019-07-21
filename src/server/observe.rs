pub trait Observable<'a, T: 'a> {
    fn subscribe_observer(&mut self, observer: &'a impl Observer<T>);

    fn get_value(&'a self) -> &'a T;

    fn get_observers(&self) -> &[&dyn Observer<T>];

    fn notify_observers(&'a self) {
        for &observer in self.get_observers() {
            observer.notify(self.get_value());
        }
    }
}

pub trait Observer<T> {
    fn notify(&self, value: &T);
}