# React-like Project Structure

This Dioxus project follows a React application structure for better organization and maintainability.

## Directory Structure

```
src/
├── main.rs                 # Entry point
├── app.rs                  # App component and routing
│
├── components/             # Reusable UI components
│   ├── common/             # Common components (Button, Input, Card, etc.)
│   ├── layout/             # Layout components (Header, Footer, BottomNav)
│   ├── forms/              # Form components (LoginForm, RegisterForm)
│   └── ui/                 # UI-specific components (ProductCard, CategoryCard)
│
├── pages/                  # Page components (routes)
│   ├── splash.rs
│   ├── login.rs
│   ├── register.rs
│   ├── home.rs
│   └── ...
│
├── hooks/                  # Custom hooks (useAuth, useCart, etc.)
│
├── context/                # Context providers (React Context equivalent)
│   ├── auth_context.rs     # Authentication context
│   ├── cart_context.rs     # Cart context
│   └── theme_context.rs    # Theme context
│
├── api/                    # API calls and endpoints
│   ├── client.rs           # HTTP client
│   ├── auth.rs             # Auth API
│   ├── products.rs         # Products API
│   ├── cart.rs             # Cart API
│   ├── orders.rs           # Orders API
│   ├── addresses.rs        # Addresses API
│   └── wallet.rs           # Wallet API
│
├── services/               # Business logic services
│   └── storage.rs          # Local storage service
│
├── types/                  # Type definitions (equivalent to TypeScript types)
│   ├── auth.rs
│   ├── product.rs
│   ├── cart.rs
│   ├── order.rs
│   ├── category.rs
│   └── address.rs
│
├── utils/                  # Utility functions
│
├── constants/              # Constants
│   ├── api_routes.rs       # API route constants
│   └── app_constants.rs    # App constants
│
└── config/                  # Configuration
    └── app_config.rs       # App configuration
```

## Key Differences from Standard Dioxus Structure

### 1. **Types vs Models**
- Uses `types/` instead of `models/` (more React/TypeScript-like)
- Types represent data structures and API contracts

### 2. **API Layer**
- Separated API calls into `api/` directory
- Each domain has its own API file (auth, products, cart, etc.)
- Centralized HTTP client in `api/client.rs`

### 3. **Context Providers**
- React-like context system in `context/`
- Provides global state management
- Similar to React Context API

### 4. **Component Organization**
- Components organized by purpose:
  - `common/` - Reusable components
  - `layout/` - Layout components
  - `forms/` - Form components
  - `ui/` - UI-specific components

### 5. **Constants and Config**
- Separated constants from configuration
- `constants/` for static values
- `config/` for runtime configuration

## Usage Examples

### Using Context
```rust
use crate::context::use_auth;

#[component]
fn MyComponent() -> Element {
    let auth = use_auth();
    // Use auth context
}
```

### Using API
```rust
use crate::api::auth::AuthApi;
use crate::config::AppConfig;

let auth_api = AuthApi::new(AppConfig::api_base_url());
let result = auth_api.login(request).await;
```

### Using Components
```rust
use crate::components::common::{Button, Input, Card};
use crate::components::layout::Header;

#[component]
fn MyPage() -> Element {
    rsx! {
        Header { title: "My Page" }
        Card {
            Input { placeholder: "Enter text" }
            Button { "Submit" }
        }
    }
}
```

## Benefits of This Structure

1. **Familiar to React Developers** - Easy to understand for developers coming from React
2. **Better Organization** - Clear separation of concerns
3. **Scalability** - Easy to add new features and components
4. **Maintainability** - Clear structure makes code easier to maintain
5. **Reusability** - Components are organized for maximum reusability
