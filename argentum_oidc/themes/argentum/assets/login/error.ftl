<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=false; section>
    <#if section = "header">
        <div role="alert" class="alert alert-error alert-soft mt-5">
        ${kcSanitize(msg("errorTitle"))?no_esc}
        </div>
    <#elseif section = "form">
        <div id="kc-error-message" class="card bg-base-200 rounded-box grid p-5">
            <div class="instruction">${kcSanitize(message.summary)?no_esc}</div>
            <div class="pt-5">
            <a class="link link-info" href="${properties.loginUrl}">${kcSanitize(msg("goToLogin"))?no_esc}</a>
            </div>
        </div>
    </#if>
</@layout.registrationLayout>
