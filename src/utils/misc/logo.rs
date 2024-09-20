// Copyright 2019-2024 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use dialoguer::console::Term;
use tracing::info;

use crate::{networks::NetworkChain, shim::version::NetworkVersion};

/// Displays the network logo/ASCII art if available.
pub fn display_chain_logo(chain: &NetworkChain) {
    // Non-interactive terminal - don't print the logo to avoid polluting prod-like environments.
    if !Term::stderr().is_term() {
        return;
    }
    match chain {
        NetworkChain::Butterflynet => {
            reveal_butterfly_logo();
        }
        NetworkChain::Mainnet | NetworkChain::Calibnet | NetworkChain::Devnet(_) => {
            // no logo for these networks (yet)
        }
    };
}

pub fn reveal_upgrade_logo(network_version: NetworkVersion) {
    // Non-interactive terminal - don't print the logo to avoid polluting prod-like environments.
    if !Term::stderr().is_term() {
        return;
    }
    match network_version {
        NetworkVersion::V23 => reveal_waffle_upgrade(),
        _ => reveal_three_trees(),
    }
}

/// Reveals three trees arranged in an order that resemble the forest logo.
/// To be used at anyone's convenience.
fn reveal_three_trees() {
    info!(
        r###"
                           ███████
                     ███████████████████
                     ███████████████████
                     ███████████████████
                       ▄█████████████▄
                    ▄███████████████████▄
                 ███████████████████████████
                 ██████ █████████████ ██████
                     ▄▄███████████████▄▄
               ███████████████████████████████
              ████████████ ███████ ████████████
             ██████████▀   ███████   ▀██████████
                           ███████
             ███████       ███████       ███████
       ███████████████████ ███████ ███████████████████
       ███████████████████         ███████████████████
       ███████████████████         ███████████████████
         ▄█████████████▄             ▄█████████████▄
      ▄███████████████████▄       ▄███████████████████▄
   ███████████████████████████ ███████████████████████████
   ██████ █████████████ ██████ ██████ █████████████ ██████
       ▄▄███████████████▄▄         ▄▄███████████████▄▄
  █████████████████████████████████████████████████████████
 ███████████ ███████ ███████████████████ ███████ ███████████
█████████▀   ███████   ▀█████████████▀   ███████   ▀█████████
             ███████                     ███████
             ███████                     ███████
             ███████                     ███████
"###
    );
}

/// Reveals a beautiful Belgian waffle. A keen eye may notice that the waffle is built out of
/// smaller waffles.
fn reveal_waffle_upgrade() {
    info!(
        r###"
                                                                                
                                       ##                                       
                                    #### ###                                    
                                    ##    ##                                    
                      ######        ##    ##        ######                      
                     ##    ##      ###    ###      ##    ##                     
                    ###    ###########    ###########    ###                    
                   ####    ##       ##    ##       ##    ####                   
               ########    ###########    ###########    ########               
             #########      #########      #########      #########             
            ##                                                    ##            
            ###                                                  ###            
              #########    ###########    ###########    #########              
                 ##  ##    ##       ##    ##       ##    ##  ##                 
                 ##  ##    ##       ##    ##       ##    ##  ##                 
                 ##  ##    ##       ##    ##       ##    ##  ###                
              ####   ##    ##       ##    ##       ##    ##   ####              
         ##############    ###########    ###########    ##############         
        ##                                                            ##        
        ###                                                           ##        
         ##############    ###########    ###########    ##############         
               ###   ##    ##       ##    ##       ##    ##   ###               
                 ##  ##    ##       ##    ##       ##    ##  ##                 
                 ##  ##    ##       ##    ##       ##    ##  ##                 
                ###  ##    ##       ##    ##       ##    ##   ##                
             ##########    ###########    ###########    ##########             
            ##                                                    ##            
            ##                                                    ##            
             ##########    ###########    ###########    ##########             
                 ######    ##       ##    ##       ##    ######                 
                    ###    ## ####  ##    ##  #### ##    ###                    
                     ##    ####  #####    #####  ####    ##                     
                     ##    ##       ##    ###      ##    ##                     
                      ######        ##    ##        ######                      
                                    ##    ##                                    
                                     ######                       

        "###
    );
}

/// Reveals a Butterfly logo. A beautiful butterfly that will not enjoy its life for long.
fn reveal_butterfly_logo() {
    info!(
        r###"
               ,,_
           zd$$??=
         z$$P? F:`c,                _
        d$$, `c'cc$$i           ,cd$?R
       $$$$ cud$,?$$$i       ,=P"2?z "
        $" " ?$$$,?$$$.    ,-''`>, bzP
         'cLdb,?$$,?$$$   ,h' "I$'J$P
      ... `?$$$,"$$,`$$h  $$PxrF'd$"
    d$PP""?-,"?$$,?$h`$$,,$$'$F44"
    ?,,_`=4c,?=,"?hu?$`?L4$'? '
       `""?==""=-"" `""-`'_,,,,
               .ccu?m?e?JC,-,"=?
                         """=='?"
    "###
    );
}
