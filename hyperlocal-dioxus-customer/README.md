# HyperLocal MultiVendor E-Commerce - Dioxus Version

This is a Rust-based web application built with Dioxus, converted from the original Flutter/Dart implementation.

## Features

- **Authentication**: Login, Register, OTP Verification
- **Product Browsing**: Categories, Product Listing, Product Details
- **Shopping Cart**: Add to cart, update quantities, remove items
- **Orders**: Order management and tracking
- **User Profile**: Profile management
- **Wallet**: Wallet balance and transactions
- **Wishlist**: Save favorite products
- **Address Management**: Manage delivery addresses
- **Payment Options**: Multiple payment gateways
- **Nearby Stores**: Find stores near you

## Project Structure

```
src/
├── main.rs           # Application entry point
├── app.rs            # Router configuration
├── models/            # Data models (Auth, Product, Cart, Order, etc.)
├── services/          # API services and storage
├── pages/             # Page components
├── components/        # Reusable UI components
├── hooks/            # Custom hooks
└── utils/            # Utility functions
```

## Setup

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Dioxus CLI**:
   ```bash
   cargo install dioxus-cli
   ```

3. **Configure API Base URL**:
   - Update `YOUR_DOMAIN_BASE_URL` in the service files with your actual API base URL
   - Files to update:
     - `src/pages/login.rs`
     - `src/pages/register.rs`
     - `src/pages/home.rs`
     - `src/pages/cart.rs`
     - `src/pages/product_detail.rs`
     - `src/pages/product_listing.rs`

4. **Run the development server**:
   ```bash
   dx serve
   ```

5. **Build for production**:
   ```bash
   dx build --release
   ```

## Dependencies

- **dioxus**: UI framework
- **dioxus-web**: Web platform support
- **serde**: Serialization/deserialization
- **reqwest**: HTTP client
- **gloo-storage**: Browser storage
- **gloo-timers**: Async timers

## Key Differences from Flutter Version

1. **State Management**: Uses Dioxus signals instead of BLoC pattern
2. **Routing**: Uses Dioxus Router instead of GoRouter
3. **Storage**: Uses gloo-storage (localStorage) instead of Hive
4. **API Calls**: Uses reqwest instead of Dio
5. **UI**: Uses RSX (similar to JSX) instead of Flutter widgets

## Development Notes

- The app uses Tailwind CSS classes for styling (via class attributes)
- API responses are parsed using serde_json
- Authentication tokens are stored in localStorage
- All async operations use Rust's async/await with spawn

## TODO

- [ ] Complete product detail page implementation
- [ ] Add image loading and caching
- [ ] Implement cart sync functionality
- [ ] Add error handling and toast notifications
- [ ] Complete all remaining pages
- [ ] Add loading states and skeletons
- [ ] Implement search functionality
- [ ] Add filters and sorting
- [ ] Complete payment integration
- [ ] Add address picker with maps
- [ ] Implement order tracking

## License

Same as the original Flutter project.
