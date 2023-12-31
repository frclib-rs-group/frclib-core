use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn dimensions(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    //expect (ident operator ident = ident)
    let mut iter = input.into_iter();
    let a_name = iter.next().unwrap();
    let operator = iter.next().unwrap();
    let b_name = iter.next().unwrap();
    let eq = iter.next().unwrap();
    let c_name = iter.next().unwrap();
    let a_name = match a_name {
        proc_macro2::TokenTree::Ident(ident) => ident,
        _ => panic!("expected ident"),
    };
    let b_name = match b_name {
        proc_macro2::TokenTree::Ident(ident) => ident,
        _ => panic!("expected ident"),
    };
    let c_name = match c_name {
        proc_macro2::TokenTree::Ident(ident) => ident,
        _ => panic!("expected ident"),
    };
    let operator = match operator {
        proc_macro2::TokenTree::Punct(ident) => ident,
        _ => panic!("expected punct"),
    };
    let eq = match eq {
        proc_macro2::TokenTree::Punct(ident) => ident,
        _ => panic!("expected punct"),
    };

    if eq.as_char() != '=' {
        panic!("expected =");
    }

    let impl_block = match operator.as_char() {
        '*' => {
            quote! {
                impl std::ops::Mul<#b_name> for #a_name {
                    type Output = #c_name;
                    fn mul(self, rhs: #b_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                impl std::ops::Mul<#a_name> for #b_name {
                    type Output = #c_name;
                    fn mul(self, rhs: #a_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Mul<&#b_name> for #a_name {
                    type Output = #c_name;
                    fn mul(self, rhs: &#b_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Mul<#a_name> for &#b_name {
                    type Output = #c_name;
                    fn mul(self, rhs: #a_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Mul<#b_name> for &#a_name {
                    type Output = #c_name;
                    fn mul(self, rhs: #b_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Mul<&#a_name> for #b_name {
                    type Output = #c_name;
                    fn mul(self, rhs: &#a_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Mul<&#b_name> for &#a_name {
                    type Output = #c_name;
                    fn mul(self, rhs: &#b_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Mul<&#a_name> for &#b_name {
                    type Output = #c_name;
                    fn mul(self, rhs: &#a_name) -> Self::Output {
                        #c_name::from(self.0 * rhs.0)
                    }
                }

                //other order
                impl std::ops::Div<#a_name> for #c_name {
                    type Output = #b_name;
                    fn div(self, rhs: #a_name) -> Self::Output {
                        #b_name::from(self.0 / rhs.0)
                    }
                }
                impl std::ops::Div<#b_name> for #c_name {
                    type Output = #a_name;
                    fn div(self, rhs: #b_name) -> Self::Output {
                        #a_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<&#a_name> for #c_name {
                    type Output = #b_name;
                    fn div(self, rhs: &#a_name) -> Self::Output {
                        #b_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<#a_name> for &#c_name {
                    type Output = #b_name;
                    fn div(self, rhs: #a_name) -> Self::Output {
                        #b_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<&#a_name> for &#c_name {
                    type Output = #b_name;
                    fn div(self, rhs: &#a_name) -> Self::Output {
                        #b_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<&#b_name> for #c_name {
                    type Output = #a_name;
                    fn div(self, rhs: &#b_name) -> Self::Output {
                        #a_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<#b_name> for &#c_name {
                    type Output = #a_name;
                    fn div(self, rhs: #b_name) -> Self::Output {
                        #a_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<&#b_name> for &#c_name {
                    type Output = #a_name;
                    fn div(self, rhs: &#b_name) -> Self::Output {
                        #a_name::from(self.0 / rhs.0)
                    }
                }
            }
        }
        '/' => {
            quote! {
                impl std::ops::Div<#b_name> for #a_name {
                    type Output = #c_name;
                    fn div(self, rhs: #b_name) -> Self::Output {
                        #c_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<&#b_name> for #a_name {
                    type Output = #c_name;
                    fn div(self, rhs: &#b_name) -> Self::Output {
                        #c_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<#b_name> for &#a_name {
                    type Output = #c_name;
                    fn div(self, rhs: #b_name) -> Self::Output {
                        #c_name::from(self.0 / rhs.0)
                    }
                }
                #[cfg(feature = "with-ref-ops")]
                impl std::ops::Div<&#b_name> for &#a_name {
                    type Output = #c_name;
                    fn div(self, rhs: &#b_name) -> Self::Output {
                        #c_name::from(self.0 / rhs.0)
                    }
                }
            }
        }
        _ => panic!("expected * /"),
    };

    output.extend(impl_block);

    output
}