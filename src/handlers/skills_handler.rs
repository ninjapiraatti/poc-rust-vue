use crate::errors::ServiceError;
use crate::models::skills::Pool;
use crate::models::users::LoggedUser;
use crate::repositories::*;
use actix_web::{error::BlockingError, web, HttpResponse};
use log::trace;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ScopeLevelSwapDirection {
	Better,
	Worse,
}

impl From<&ScopeLevelSwapDirection> for skillscopelevels_repository::ScopeLevelSwapDirection {
	fn from(e: &ScopeLevelSwapDirection) -> Self {
		match e {
			ScopeLevelSwapDirection::Better => Self::Better,
			ScopeLevelSwapDirection::Worse => Self::Worse,
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct SkillData {
	pub label: String,
	pub skillcategory_id: uuid::Uuid,
	pub skillscope_id: uuid::Uuid,
}
#[derive(Deserialize, Debug)]
pub struct CategoryData {
	pub label: String,
	pub parent_id: Option<uuid::Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct ScopeData {
	pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct ScopeLevelData {
	pub label: String,
	pub skillscope_id: uuid::Uuid,
	pub percentage: Option<i32>,
	pub swap_direction: Option<ScopeLevelSwapDirection>,
}

pub async fn get_all_skills(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting all skills: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skills_repository::query_all_skills(&pool)).await;

	match res {
		Ok(skills) => {
			if skills.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&skills));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_skillscopes(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting skill scopes: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skillscopes_repository::query_skill_scopes(&pool)).await;

	match res {
		Ok(skillscopes) => {
			if skillscopes.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&skillscopes));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_skill_levels(pool: web::Data<Pool>, _logged_user: LoggedUser) -> Result<HttpResponse, ServiceError> {
	trace!("Getting skill levels: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skillscopelevels_repository::query_skill_levels(&pool)).await;

	match res {
		Ok(skill_levels) => {
			if skill_levels.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&skill_levels));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn get_skill_categories(
	pool: web::Data<Pool>,
	_logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!("Getting skill categories: logged_user= {:#?}", &_logged_user);
	let res = web::block(move || skillcategories_repository::query_skill_categories(pool)).await;

	match res {
		Ok(categories) => {
			if categories.is_empty() == false {
				return Ok(HttpResponse::Ok().json(&categories));
			}
			Err(ServiceError::Empty)
		}
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill_category(
	categorydata: web::Json<CategoryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Creating a skill category: categorydata = {:#?} logged_user = {:#?}",
		&categorydata,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		skillcategories_repository::create_skill_category(
			categorydata.label.clone(),
			categorydata.parent_id,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_skill_category(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a skill category: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skillcategories_repository::delete_skill_category(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_skill_category(
	uuid_data: web::Path<String>,
	categorydata: web::Json<CategoryData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Creating a skill categories: categorydata = {:#?} logged_user = {:#?}",
		&categorydata,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || {
		skillcategories_repository::update_skill_categories(
			id,
			categorydata.label.clone(),
			categorydata.parent_id.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(skillcategory) => Ok(HttpResponse::Ok().json(&skillcategory)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_skill_scope_level(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a skill scope level: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skillscopelevels_repository::delete_skill_scope_level(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill(
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Creating a skill: skilldata = {:#?} logged_user = {:#?}",
		&skilldata,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		skills_repository::create_skill(
			skilldata.label.clone(),
			skilldata.skillcategory_id,
			skilldata.skillscope_id,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_skill(
	uuid_data: web::Path<String>,
	skilldata: web::Json<SkillData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Creating a skill: skilldata = {:#?} logged_user = {:#?}",
		&skilldata,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || {
		skills_repository::update_skill(
			id,
			skilldata.label.clone(),
			skilldata.skillcategory_id,
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(skill) => Ok(HttpResponse::Ok().json(&skill)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_skill_scope(
	uuid_data: web::Path<String>,
	scopedata: web::Json<ScopeData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating a scope: scopedata = {:#?} logged_user = {:#?}",
		&scopedata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || {
		skillscopes_repository::update_skill_scope(id, scopedata.label.clone(), logged_user.email, &pool)
	})
	.await;
	match res {
		Ok(scope) => Ok(HttpResponse::Ok().json(&scope)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill_scope(
	scopedata: web::Json<ScopeData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Creating a skill scope: scopedata = {:#?} logged_user = {:#?}",
		&scopedata,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		skillscopes_repository::create_skill_scope(scopedata.label.clone(), logged_user.email, &pool)
	})
	.await;
	match res {
		Ok(skill_scope) => Ok(HttpResponse::Ok().json(&skill_scope)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_skill_scope(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a skill scope: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skillscopes_repository::delete_skill_scope(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn create_skill_scope_level(
	scopeleveldata: web::Json<ScopeLevelData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Creating a skill scope level: scopeleveldata = {:#?} logged_user = {:#?}",
		&scopeleveldata,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let res = web::block(move || {
		skillscopelevels_repository::create_skill_scope_level(
			scopeleveldata.label.clone(),
			scopeleveldata.percentage,
			scopeleveldata.skillscope_id.clone(),
			logged_user.email,
			&pool,
		)
	})
	.await;
	match res {
		Ok(skill_scope_level) => Ok(HttpResponse::Ok().json(&skill_scope_level)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn update_skill_scope_level(
	uuid_data: web::Path<String>,
	scopeleveldata: web::Json<ScopeLevelData>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Updating a scope level: scopeleveldata = {:#?} logged_user = {:#?}",
		&scopeleveldata,
		&logged_user
	);

	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;
	let a: Option<skillscopelevels_repository::ScopeLevelSwapDirection> = match &scopeleveldata.swap_direction {
		Some(b) => Some(b.into()),
		None => None,
	};

	let res = web::block(move || {
		skillscopelevels_repository::update_skill_scope_level(
			id,
			scopeleveldata.label.clone(),
			scopeleveldata.percentage.clone(),
			logged_user.email,
			a,
			&pool,
		)
	})
	.await;
	match res {
		Ok(scopelevel) => Ok(HttpResponse::Ok().json(&scopelevel)),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}

pub async fn delete_skill(
	uuid_data: web::Path<String>,
	pool: web::Data<Pool>,
	logged_user: LoggedUser,
) -> Result<HttpResponse, ServiceError> {
	trace!(
		"Deleting a skill: uuid_data = {:#?} logged_user = {:#?}",
		&uuid_data,
		&logged_user
	);

	// todo: create a macro to simplify this
	if logged_user.isadmin == false {
		return Err(ServiceError::AdminRequired);
	}

	let id = uuid::Uuid::parse_str(&uuid_data.into_inner())?;

	let res = web::block(move || skills_repository::delete_skill(id, &pool)).await;
	match res {
		Ok(_) => Ok(HttpResponse::Ok().finish()),
		Err(err) => match err {
			BlockingError::Error(service_error) => Err(service_error.into()),
			BlockingError::Canceled => Err(ServiceError::InternalServerError),
		},
	}
}
