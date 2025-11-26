<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=false; section>
    <#if section = "header">
        <#if messageHeader??>
            ${kcSanitize(msg("${messageHeader}"))?no_esc}
        <#else>
            <div role="alert" class="alert alert-info alert-soft mt-5">
                ${message.summary}
            </div>
        </#if>
    <#elseif section = "form">
    <div id="kc-info-message" class="card bg-base-200 rounded-box grid p-5">
        <p class="instruction">${message.summary}
            <#if requiredActions??>
                <#list requiredActions>: <b><#items as reqActionItem>${kcSanitize(msg("requiredAction.${reqActionItem}"))?no_esc}<#sep>, </#items></b></#list><#else>
            </#if>
        </p>
        <#if skipLink??>
        <#else>
            <#if pageRedirectUri?has_content>
                <p class="pt-10"><a href="${pageRedirectUri}">${kcSanitize(msg("backToApplication"))?no_esc}</a></p>
            <#elseif actionUri?has_content>
                <p class="pt-5"><a href="${actionUri}" class="btn btn-primary">${kcSanitize(msg("proceedWithAction"))?no_esc}</a></p>
            <#elseif (client.baseUrl)?has_content>
                <p class="pt-10"><a href="${client.baseUrl}">${kcSanitize(msg("backToApplication"))?no_esc}</a></p>
            </#if>
        </#if>
        <div class="mt-5">
            <a class="link link-info" href="${properties.loginUrl}">${kcSanitize(msg("goToLogin"))?no_esc}</a>
        </div>
        
    </div>
    </#if>
</@layout.registrationLayout>
