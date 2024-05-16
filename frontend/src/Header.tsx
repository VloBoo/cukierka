import React from 'react';

class Header extends React.Component {
    render(): React.ReactNode {
        return (
            <div className="row">
                <div className="col-12">
                    <nav className="navbar navbar-expand-xl navbar-light bg-light">
                        <a className="navbar-brand" href="#">
                            <i className="fas fa-3x fa-circle-notch tm-site-icon"></i>
                            <h1 className="tm-site-title mb-0">Cukierka</h1>
                        </a>
                        <button className="navbar-toggler ml-auto mr-0" type="button" data-toggle="collapse"
                            data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent"
                            aria-expanded="false" aria-label="Toggle navigation">
                            <span className="navbar-toggler-icon"></span>
                        </button>

                        <div className="collapse navbar-collapse" id="navbarSupportedContent">
                            <ul className="navbar-nav mx-auto">
                                <li className="nav-item">
                                    <a className="nav-link" href="/">Главная</a>
                                </li>
                                <li className="nav-item">
                                    <a className="nav-link active" id="nav-resume" href="resume-add.html">Резюме
                                        <span className="sr-only">(current)</span>
                                    </a>
                                </li>
                                <li className="nav-item">
                                    <a className="nav-link" href="vacancies.html">Вакансии</a>
                                </li>
                                <li className="nav-item">
                                    <a className="nav-link" href="my-vacancies.html">Мои вакансии</a>
                                </li>
                                <li className="nav-item">
                                    <a className="nav-link" id="nav-my-account" href="account.html">Аккаунт</a>
                                </li>
                                <li className="nav-item" id="nav-table-block" style={{ display: 'none' }}>
                                    <a className="nav-link" href="/table/index.html">Таблица</a>
                                </li>
                            </ul>
                            <ul className="navbar-nav">
                                <li className="nav-item">
                                    <a className="nav-link d-flex">
                                        <i className="far fa-user mr-2 tm-logout-icon"></i>
                                        <span>Выйти</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </nav>
                </div>
            </div>
        );
    }
}

export default Header;