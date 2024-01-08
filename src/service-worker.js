// service-worker.js
self.addEventListener('push', event => {
    console.log('HERE');
    console.log('Raw Data:', event.data.text());

    const data = event.data.json();
    console.log('Push event!! ', data);
    const title = data.title;
    const options = {
        body: data.body,
        icon: data.icon,
        badge: data.badge
    };
    event.waitUntil(self.registration.showNotification(title, options));
});

self.addEventListener('notificationclick', function (event) {
    event.notification.close();
    // Handle the notification click
});
