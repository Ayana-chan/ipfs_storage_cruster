#[allow(unused_imports)]
use tracing::{error, debug, warn, info};
use crate::{IpfsClientError, dtos, ReqwestIpfsClient, IpfsClientResult};

impl ReqwestIpfsClient {
    /// Get file from IPFS gateway.
    #[cfg(not(feature = "no_gateway"))]
    pub async fn get_file_by_gateway(&self, cid: &str, file_name: Option<&str>) -> IpfsClientResult<reqwest::Response> {
        let url = format!("http://{addr}/ipfs/{cid}?filename={file_name}&download=true",
                          addr = &self.gateway_address,
                          cid = cid,
                          file_name = file_name.unwrap_or(cid)
        );

        let res = self.client
            .get(url)
            .send()
            .await.map_err(|_e| {
            error!("Send gateway request failed. msg: {:?}", _e);
            IpfsClientError::SendRequestFailed
        })?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                info!("Success get file. cid: {}", cid);
                Ok(res)
            }
            reqwest::StatusCode::NOT_FOUND => {
                error!("IPFS gateway not found");
                Err(IpfsClientError::NotFound)
            }
            reqwest::StatusCode::BAD_REQUEST => {
                error!("IPFS gateway responds bad request");
                Err(IpfsClientError::BadRequest)
            }
            _ => {
                warn!("IPFS gateway responds unhandled status code: {}", status);
                Err(IpfsClientError::UnknownStatusCode)
            }
        }
    }

    /// Get IPFS node's basic information.
    pub async fn get_id_info(&self) -> IpfsClientResult<dtos::IdResponse> {
        // TODO format arg无效
        let url_content = "/id";
        let res = self.ipfs_rpc_request(url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                let id_res: dtos::IdResponse = res.json().await.map_err(|_e| {
                    error!("Unexpected response body. msg: {:?}", _e);
                    IpfsClientError::UnexpectedResponseBody
                })?;
                info!("Success get id info. peer id: {}", id_res.id);
                Ok(id_res)
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// List all recursive pins that is pinned
    pub async fn list_recursive_pins_pinned(&self, with_pin_name: bool) -> IpfsClientResult<dtos::ListPinsResponse> {
        let url_content = if with_pin_name {
            "/pin/ls?type=recursive&names=true"
        } else {
            "/pin/ls?type=recursive"
        };
        let res = self.ipfs_rpc_request(url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                let pins = res.json().await.map_err(|_e| {
                    error!("Unexpected response body. msg: {:?}", _e);
                    IpfsClientError::UnexpectedResponseBody
                })?;
                info!("Success list recursive pins that is pinned");
                Ok(pins)
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Get a pin.
    pub async fn get_one_pin(&self, cid: &str, with_pin_name: bool) -> IpfsClientResult<Option<dtos::PinsInfoInList>> {
        let url_content = format!("/pin/ls?arg={cid}&names={with_pin_name}",
                                  cid = cid, with_pin_name = with_pin_name);
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                // pinned
                let pins: dtos::ListPinsResponse = res.json().await.map_err(|_e| {
                    error!("Unexpected response body. msg: {:?}", _e);
                    IpfsClientError::UnexpectedResponseBody
                })?;

                if pins.keys.len() != 1 {
                    error!("Unexpected response body. Should only respond one pin");
                    return Err(IpfsClientError::UnexpectedResponseBody);
                }

                info!("Success get one pin. cid: {}", cid);
                let pin_info = pins.keys.into_values().next().unwrap();
                Ok(Some(pin_info))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                // might not pinned
                let error_response: dtos::ErrorResponse = res.json().await.map_err(|_e| {
                    Self::handle_rpc_status_code_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                })?;

                if error_response.message.contains("is not pinned") {
                    // really not pinned
                    Ok(None)
                } else {
                    Err(Self::handle_rpc_status_code_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Add a recursive pin.
    pub async fn add_pin_recursive(&self, cid: &str, pin_name: Option<&str>) -> IpfsClientResult<()> {
        let pin_name = pin_name.unwrap_or("untitled");

        let url_content = format!("/pin/add?arg={cid}&name={pin_name}",
                                  cid = cid,
                                  pin_name = pin_name,
        );
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                info!("Success add pin. cid: {}, pin_name: {}", cid, pin_name);
                Ok(())
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Remove a recursive pin.
    pub async fn remove_pin_recursive(&self, cid: &str) -> IpfsClientResult<()> {
        let url_content = format!("/pin/rm?arg={cid}",
                                  cid = cid,
        );
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                info!("Success remove pin. cid: {}", cid);
                Ok(())
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Add an IPFS node to bootstrap list by ip address, port and peer id.
    pub async fn bootstrap_add(&self, ip: &str, port: &str, peer_id: &str) -> IpfsClientResult<()> {
        let multi_addr = format!("/ip4/{}/tcp/{}/p2p/{}", ip, port, peer_id);
        let url_content = format!("/bootstrap/add?arg={multi_addr}",
        multi_addr = multi_addr);
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                info!("Success add bootstrap. multi_addr: {}", multi_addr);
                Ok(())
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }
}